use std::{collections::HashMap, println};

use crate::{
    common::src::{AnyProcessor, AnySrc, Src, SrcKind},
    core::features::{FeatureValue, Features, IncrementalFeatureBuilder},
    core::{
        context::Context,
        incremental::{Incremental, IncrementalDefault},
        trend::Trend,
    },
    statistics::normalization::rescale,
    strategy::trade::{StrategySignal, TradeDirection},
    ta::{
        cross::Cross,
        cross_over_threshold::CrossOverThreshold,
        cross_under_threshold::CrossUnderThreshold,
        highest_bars::HighestBars,
        lowest_bars::LowestBars,
        moving_average::{Ma, MaKind},
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

impl Incremental<(), f64> for RelativeStrengthIndex {
    fn next(&mut self, _: ()) -> f64 {
        let src = self.config.src.next(());
        return self.rsi.next(src);
    }
}

pub static RELATIVE_STRENGTH_INDEX_THRESHOLD_OVERSOLD: f64 = 30.0;
pub static RELATIVE_STRENGTH_INDEX_THRESHOLD_OVERBOUGHT: f64 = 70.0;

#[derive(Debug, Clone)]
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

impl Incremental<f64, StrategySignal> for RelativeStrengthIndexStrategy {
    fn next(&mut self, rsi: f64) -> StrategySignal {
        let is_cross_over = self.cross_over.next(rsi);
        let is_cross_under = self.cross_under.next(rsi);

        if is_cross_over {
            return StrategySignal::Long;
        }
        if is_cross_under {
            return StrategySignal::Short;
        }
        return StrategySignal::Hold;
    }
}
