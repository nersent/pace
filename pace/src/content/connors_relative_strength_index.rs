use crate::{
    common::src::{AnySrc, Src, SrcKind},
    core::{
        context::Context,
        incremental::{Incremental, IncrementalDefault},
    },
    pinescript::common::ps_nz,
    strategy::trade::TradeDirection,
    ta::{
        cross::Cross,
        cross_over_threshold::CrossOverThreshold,
        cross_under_threshold::CrossUnderThreshold,
        highest_bars::HighestBars,
        lowest_bars::LowestBars,
        moving_average::{AnyMa, Ma, MaKind},
        percent_rank::Prank,
        rate_of_change::Roc,
        relative_strength_index::Rsi,
    },
};

pub static CONNORS_RELATIVE_STRENGTH_INDEX_MIN_VALUE: f64 = 0.0;
pub static CONNORS_RELATIVE_STRENGTH_INDEX_MAX_VALUE: f64 = 100.0;

pub struct ConnorsRelativeStrengthIndexConfig {
    pub length_rsi: usize,
    pub length_up_down: usize,
    pub length_roc: usize,
    pub src: AnySrc,
}

impl IncrementalDefault for ConnorsRelativeStrengthIndexConfig {
    fn default(ctx: Context) -> Self {
        return Self {
            length_rsi: 3,
            length_up_down: 2,
            length_roc: 100,
            src: Src::new(ctx.clone(), SrcKind::Close).to_box(),
        };
    }
}

/// Ported from https://www.tradingview.com/chart/?solution=43000502017
pub struct ConnorsRelativeStrengthIndex {
    pub config: ConnorsRelativeStrengthIndexConfig,
    pub ctx: Context,
    prev_src: Option<f64>,
    prev_ud: Option<f64>,
    rsi: Rsi,
    up_down_rsi: Rsi,
    percent_rank: Prank,
    roc: Roc,
}

impl ConnorsRelativeStrengthIndex {
    pub fn new(ctx: Context, config: ConnorsRelativeStrengthIndexConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            rsi: Rsi::new(ctx.clone(), config.length_rsi),
            up_down_rsi: Rsi::new(ctx.clone(), config.length_up_down),
            percent_rank: Prank::new(ctx.clone(), config.length_roc),
            roc: Roc::new(ctx.clone(), 1),
            config,
            prev_src: None,
            prev_ud: None,
        };
    }

    fn compute_up_down(
        src: Option<f64>,
        prev_src: Option<f64>,
        prev_ud: Option<f64>,
    ) -> Option<f64> {
        if prev_src == src {
            return Some(0.0);
        }
        let prev_ud = ps_nz(prev_ud);
        if src.is_some() && prev_src.is_some() && src.unwrap() > prev_src.unwrap() {
            if prev_ud <= 0.0 {
                return Some(1.0);
            } else {
                return Some(prev_ud + 1.0);
            }
        } else if prev_ud >= 0.0 {
            return Some(-1.0);
        } else {
            return Some(prev_ud - 1.0);
        }
    }
}

impl Incremental<(), Option<f64>> for ConnorsRelativeStrengthIndex {
    fn next(&mut self, _: ()) -> Option<f64> {
        let src = self.config.src.next(());

        let rsi = self.rsi.next(src);

        let up_down = Self::compute_up_down(src, self.prev_src, self.prev_ud);
        let up_down_rsi = self.up_down_rsi.next(up_down);

        let roc = self.roc.next(src);
        let percent_rank = self.percent_rank.next(roc);

        let crsi = match (rsi, up_down_rsi, percent_rank) {
            (Some(rsi), Some(up_down_rsi), Some(percent_rank)) => {
                Some((rsi + up_down_rsi + percent_rank) / 3.0)
            }
            _ => None,
        };

        self.prev_ud = up_down;
        self.prev_src = src;

        return crsi;
    }
}

pub static CONNORS_RELATIVE_STRENGTH_INDEX_THRESHOLD_OVERSOLD: f64 = 20.0;
pub static CONNORS_RELATIVE_STRENGTH_INDEX_THRESHOLD_OVERBOUGHT: f64 = 80.0;

pub struct ConnorsRelativeStrengthIndexStrategyConfig {
    pub threshold_oversold: f64,
    pub threshold_overbought: f64,
}

impl Default for ConnorsRelativeStrengthIndexStrategyConfig {
    fn default() -> Self {
        return Self {
            threshold_oversold: CONNORS_RELATIVE_STRENGTH_INDEX_THRESHOLD_OVERSOLD,
            threshold_overbought: CONNORS_RELATIVE_STRENGTH_INDEX_THRESHOLD_OVERBOUGHT,
        };
    }
}

/// Custom Connors Relative Strength Index Strategy. May be incorrect.
pub struct ConnorsRelativeStrengthIndexStrategy {
    pub config: ConnorsRelativeStrengthIndexStrategyConfig,
    pub ctx: Context,
    cross_overbought: CrossOverThreshold,
    cross_oversold: CrossUnderThreshold,
}

impl ConnorsRelativeStrengthIndexStrategy {
    pub fn new(ctx: Context, config: ConnorsRelativeStrengthIndexStrategyConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            cross_overbought: CrossOverThreshold::new(ctx.clone(), config.threshold_oversold),
            cross_oversold: CrossUnderThreshold::new(ctx.clone(), config.threshold_overbought),
            config,
        };
    }
}

impl Incremental<Option<f64>, Option<TradeDirection>> for ConnorsRelativeStrengthIndexStrategy {
    fn next(&mut self, rsi: Option<f64>) -> Option<TradeDirection> {
        let is_cross_over = self.cross_overbought.next(rsi);
        let is_cross_under = self.cross_oversold.next(rsi);

        let result = if is_cross_over {
            Some(TradeDirection::Long)
        } else if is_cross_under {
            Some(TradeDirection::Short)
        } else {
            None
        };

        return result;
    }
}
