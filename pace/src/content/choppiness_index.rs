use std::collections::HashMap;

use crate::{
    common::src::{AnySrc, Src, SrcKind},
    core::{
        context::Context,
        features::{FeatureValue, Features, IncrementalFeatureBuilder},
        incremental::{Incremental, IncrementalDefault},
        trend::Trend,
    },
    statistics::normalization::rescale,
    strategy::trade::TradeDirection,
    ta::{
        average_true_range::Atr, cross::Cross, cross_over_threshold::CrossOverThreshold,
        cross_under_threshold::CrossUnderThreshold, highest::Highest, highest_bars::HighestBars,
        lowest::Lowest, lowest_bars::LowestBars, sum::Sum,
    },
    utils::float::Float64Utils,
};

pub static CHOPPINESS_INDEX_MIN_VALUE: f64 = 0.0;
pub static CHOPPINESS_INDEX_MAX_VALUE: f64 = 100.0;

pub struct ChoppinessIndexConfig {
    pub length: usize,
}

impl Default for ChoppinessIndexConfig {
    fn default() -> Self {
        Self { length: 14 }
    }
}

/// Choppiness Index Indicator.
///
/// Ported from https://www.tradingview.com/chart/?solution=43000501980
pub struct ChoppinessIndex {
    pub config: ChoppinessIndexConfig,
    pub ctx: Context,
    atr: Atr,
    atr_sum: Sum,
    log10_length: f64,
    highest: Highest,
    lowest: Lowest,
}

impl ChoppinessIndex {
    pub fn new(ctx: Context, config: ChoppinessIndexConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            atr: Atr::new(ctx.clone(), 1),
            atr_sum: Sum::new(ctx.clone(), config.length),
            log10_length: f64::log10(config.length as f64),
            highest: Highest::new(ctx.clone(), config.length),
            lowest: Lowest::new(ctx.clone(), config.length),
            config,
        };
    }
}

impl Incremental<(), f64> for ChoppinessIndex {
    fn next(&mut self, _: ()) -> f64 {
        let atr = self.atr.next(());
        let atr_sum = self.atr_sum.next(atr);

        let highest = self.highest.next(self.ctx.bar.high());
        let lowest = self.lowest.next(self.ctx.bar.low());

        let chop = 100.0 * f64::log10(atr_sum / (highest - lowest).normalize()) / self.log10_length;

        return chop;
    }
}

pub static CHOPPINESS_INDEX_THRESHOLD_SIDEWAYS: f64 = 61.8;
pub static CHOPPINESS_INDEX_THRESHOLD_TRENDING: f64 = 38.2;
