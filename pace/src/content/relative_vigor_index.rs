use crate::{
    common::src::{AnySrc, Src, SrcKind},
    core::{
        context::Context,
        incremental::{Incremental, IncrementalDefault},
    },
    pinescript::common::ps_div,
    strategy::trade::TradeDirection,
    ta::{
        cross::{Cross, CrossMode},
        cross_over_threshold::CrossOverThreshold,
        cross_under_threshold::CrossUnderThreshold,
        highest_bars::HighestBars,
        lowest_bars::LowestBars,
        moving_average::{AnyMa, Ma, MaKind},
        sum::Sum,
        symmetrically_weighted_moving_average::Swma,
    },
};

pub static RELATIVE_VIGOR_INDEX_MIN_VALUE: f64 = -1.0;
pub static RELATIVE_VIGOR_INDEX_MAX_VALUE: f64 = 1.0;

pub struct RelativeVigorIndexConfig {
    pub length: usize,
}

impl IncrementalDefault for RelativeVigorIndexConfig {
    fn default(ctx: Context) -> Self {
        Self { length: 10 }
    }
}

pub struct RelativeVigorIndexData {
    pub rvi: Option<f64>,
    pub sig: Option<f64>,
}

/// Ported from https://www.tradingview.com/chart/?solution=43000591593
pub struct RelativeVigorIndex {
    pub config: RelativeVigorIndexConfig,
    pub ctx: Context,
    swma_close_open: Swma,
    swma_high_low: Swma,
    sum_close_open: Sum,
    sum_high_low: Sum,
    swma_sig: Swma,
}

impl RelativeVigorIndex {
    pub fn new(ctx: Context, config: RelativeVigorIndexConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            swma_close_open: Swma::new(ctx.clone()),
            swma_high_low: Swma::new(ctx.clone()),
            sum_close_open: Sum::new(ctx.clone(), config.length),
            sum_high_low: Sum::new(ctx.clone(), config.length),
            swma_sig: Swma::new(ctx.clone()),
            config,
        };
    }
}

impl Incremental<(), RelativeVigorIndexData> for RelativeVigorIndex {
    fn next(&mut self, _: ()) -> RelativeVigorIndexData {
        let close = self.ctx.bar.close();
        let open = self.ctx.bar.open();
        let high = self.ctx.bar.high();
        let low = self.ctx.bar.low();

        let close_open_diff = match (close, open) {
            (Some(close), Some(open)) => Some(close - open),
            _ => None,
        };

        let high_low_diff = match (high, low) {
            (Some(high), Some(low)) => Some(high - low),
            _ => None,
        };

        let close_open_swma = self.swma_close_open.next(close_open_diff);
        let high_low_swma = self.swma_high_low.next(high_low_diff);

        let close_open_sum = self.sum_close_open.next(close_open_swma);
        let high_low_sum = self.sum_high_low.next(high_low_swma);

        let rvi = ps_div(close_open_sum, high_low_sum);

        let sig = self.swma_sig.next(rvi);

        return RelativeVigorIndexData { rvi, sig };
    }
}

/// Custom Relative Vigor Index Strategy. May be incorrect.
pub struct RelativeVigorIndexStrategy {
    pub ctx: Context,
    cross: Cross,
}

impl RelativeVigorIndexStrategy {
    pub fn new(ctx: Context) -> Self {
        return Self {
            ctx: ctx.clone(),
            cross: Cross::new(ctx.clone()),
        };
    }
}

impl Incremental<&RelativeVigorIndexData, Option<TradeDirection>> for RelativeVigorIndexStrategy {
    fn next(&mut self, rvgi: &RelativeVigorIndexData) -> Option<TradeDirection> {
        let rvi_s_cross = self.cross.next((rvgi.rvi, rvgi.sig));

        let mut result: Option<TradeDirection> = None;

        if let Some(plus_minus_cross) = rvi_s_cross {
            result = match plus_minus_cross {
                CrossMode::Over => Some(TradeDirection::Long),
                CrossMode::Under => Some(TradeDirection::Short),
            }
        }

        return result;
    }
}
