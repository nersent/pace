use crate::{
    common::src::{AnySrc, Src, SrcKind},
    core::{
        context::Context,
        incremental::{Incremental, IncrementalDefault},
    },
    strategy::trade::{StrategySignal, TradeDirection},
    ta::{
        cross::Cross,
        cross_over_threshold::CrossOverThreshold,
        cross_under_threshold::CrossUnderThreshold,
        highest::Highest,
        highest_bars::HighestBars,
        lowest::Lowest,
        lowest_bars::LowestBars,
        moving_average::{Ma, MaKind},
    },
};

pub static WILLIAMS_PERCENT_RANK_MIN_VALUE: f64 = -100.0;
pub static WILLIAMS_PERCENT_RANK_MAX_VALUE: f64 = 0.0;

pub struct WilliamsPercentRankConfig {
    pub length: usize,
    pub src: AnySrc,
}

impl IncrementalDefault for WilliamsPercentRankConfig {
    fn default(ctx: Context) -> Self {
        Self {
            length: 14,
            src: Src::new(ctx.clone(), SrcKind::Close).to_box(),
        }
    }
}

/// Ported from https://www.tradingview.com/chart/?solution=43000501985
pub struct WilliamsPercentRank {
    pub config: WilliamsPercentRankConfig,
    pub ctx: Context,
    highest: Highest,
    lowest: Lowest,
}

impl WilliamsPercentRank {
    pub fn new(ctx: Context, config: WilliamsPercentRankConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            highest: Highest::new(ctx.clone(), config.length),
            lowest: Lowest::new(ctx.clone(), config.length),
            config,
        };
    }
}

impl Incremental<(), f64> for WilliamsPercentRank {
    fn next(&mut self, _: ()) -> f64 {
        let src = self.config.src.next(());
        let max = self.highest.next(self.ctx.bar.high());
        let min = self.lowest.next(self.ctx.bar.low());

        let pr = ((src - max) / (max - min)) * 100.0;

        return pr;
    }
}

pub static WILLIAMS_PERCENT_RANK_THRESHOLD_OVERSOLD: f64 = -80.0;
pub static WILLIAMS_PERCENT_RANK_THRESHOLD_OVERBOUGHT: f64 = -20.0;

pub struct WilliamsPercentRankStrategyConfig {
    pub threshold_oversold: f64,
    pub threshold_overbought: f64,
}

impl Default for WilliamsPercentRankStrategyConfig {
    fn default() -> Self {
        return Self {
            threshold_oversold: WILLIAMS_PERCENT_RANK_THRESHOLD_OVERSOLD,
            threshold_overbought: WILLIAMS_PERCENT_RANK_THRESHOLD_OVERBOUGHT,
        };
    }
}

/// Custom Williams %r Strategy. May be incorrect.
pub struct WilliamsPercentRankStrategy {
    pub config: WilliamsPercentRankStrategyConfig,
    pub ctx: Context,
    cross_overbought: CrossOverThreshold,
    cross_oversold: CrossUnderThreshold,
}

impl WilliamsPercentRankStrategy {
    pub fn new(ctx: Context, config: WilliamsPercentRankStrategyConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            cross_overbought: CrossOverThreshold::new(ctx.clone(), config.threshold_overbought),
            cross_oversold: CrossUnderThreshold::new(ctx.clone(), config.threshold_oversold),
            config,
        };
    }
}

impl Incremental<f64, StrategySignal> for WilliamsPercentRankStrategy {
    fn next(&mut self, wpr: f64) -> StrategySignal {
        let is_cross_over = self.cross_overbought.next(wpr);
        let is_cross_under = self.cross_oversold.next(wpr);

        if is_cross_over {
            return StrategySignal::Long;
        }
        if is_cross_under {
            return StrategySignal::Short;
        }
        return StrategySignal::Hold;
    }
}
