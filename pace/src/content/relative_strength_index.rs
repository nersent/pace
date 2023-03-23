use crate::{
    common::src::{AnySrc, Src, SrcKind},
    core::{
        context::Context,
        incremental::{Incremental, IncrementalDefault},
    },
    strategy::trade::TradeDirection,
    ta::{
        cross::Cross,
        cross_over_threshold::CrossOverThreshold,
        cross_under_threshold::CrossUnderThreshold,
        highest_bars::HighestBars,
        lowest_bars::LowestBars,
        moving_average::{AnyMa, Ma, MaKind},
        relative_strength_index::Rsi,
    },
};

pub static RELATIVE_STRENGTH_INDEX_MIN_VALUE: f64 = 0.0;
pub static RELATIVE_STRENGTH_INDEX_MAX_VALUE: f64 = 100.0;

pub struct RelativeStrengthIndexConfig {
    pub length: usize,
    pub src: AnySrc,
}

impl IncrementalDefault for RelativeStrengthIndexConfig {
    fn default(ctx: Context) -> Self {
        return Self {
            length: 14,
            src: Src::new(ctx.clone(), SrcKind::Close).to_box(),
        };
    }
}

/// Ported from https://www.tradingview.com/chart/?solution=43000502338
pub struct RelativeStrengthIndex {
    pub ctx: Context,
    pub config: RelativeStrengthIndexConfig,
    rsi: Rsi,
}

impl RelativeStrengthIndex {
    pub fn new(ctx: Context, config: RelativeStrengthIndexConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            rsi: Rsi::new(ctx.clone(), config.length),
            config,
        };
    }
}

impl Incremental<(), Option<f64>> for RelativeStrengthIndex {
    fn next(&mut self, _: ()) -> Option<f64> {
        let src = self.config.src.next(());
        return self.rsi.next(src);
    }
}

pub static RELATIVE_STRENGTH_INDEX_THRESHOLD_OVERSOLD: f64 = 30.0;
pub static RELATIVE_STRENGTH_INDEX_THRESHOLD_OVERBOUGHT: f64 = 70.0;

pub struct RelativeStrengthIndexStrategyConfig {
    pub threshold_oversold: f64,
    pub threshold_overbought: f64,
}

impl Default for RelativeStrengthIndexStrategyConfig {
    fn default() -> Self {
        return Self {
            threshold_oversold: RELATIVE_STRENGTH_INDEX_THRESHOLD_OVERSOLD,
            threshold_overbought: RELATIVE_STRENGTH_INDEX_THRESHOLD_OVERBOUGHT,
        };
    }
}

pub struct RelativeStrengthIndexStrategy {
    pub config: RelativeStrengthIndexStrategyConfig,
    pub ctx: Context,
    cross_over: CrossOverThreshold,
    cross_under: CrossUnderThreshold,
}

impl RelativeStrengthIndexStrategy {
    pub fn new(ctx: Context, config: RelativeStrengthIndexStrategyConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            cross_over: CrossOverThreshold::new(ctx.clone(), config.threshold_oversold),
            cross_under: CrossUnderThreshold::new(ctx.clone(), config.threshold_overbought),
            config,
        };
    }
}

impl Incremental<Option<f64>, Option<TradeDirection>> for RelativeStrengthIndexStrategy {
    fn next(&mut self, rsi: Option<f64>) -> Option<TradeDirection> {
        let cross_over = self.cross_over.next(rsi);
        let cross_under = self.cross_under.next(rsi);

        let result = if cross_over {
            Some(TradeDirection::Long)
        } else if cross_under {
            Some(TradeDirection::Short)
        } else {
            None
        };

        return result;
    }
}
