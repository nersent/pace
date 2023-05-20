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

#[derive(Debug, Clone)]
pub struct ChoppinessIndexFeatures {
    pub value: f64,
    pub trend: Option<Trend>,
    pub signal: i64,
}

impl Default for ChoppinessIndexFeatures {
    fn default() -> Self {
        return Self {
            value: f64::NAN,
            trend: None,
            signal: 0,
        };
    }
}

impl Features for ChoppinessIndexFeatures {
    fn flatten(&self) -> HashMap<String, FeatureValue> {
        let mut map: HashMap<String, FeatureValue> = HashMap::new();

        map.insert("value".to_string(), self.value.into());
        map.insert(
            "trend".to_string(),
            self.trend
                .map(|x| {
                    FeatureValue::Continous(if x == Trend::Consolidation {
                        -1.0
                    } else if x == Trend::Strong {
                        1.0
                    } else {
                        0.0
                    })
                })
                .unwrap_or(FeatureValue::Empty),
        );
        map.insert("signal".to_string(), self.signal.into());

        return map;
    }
}

pub static CHOPPINESS_INDEX_THRESHOLD_SIDEWAYS: f64 = 61.8;
pub static CHOPPINESS_INDEX_THRESHOLD_TRENDING: f64 = 38.2;

pub struct ChoppinessIndexFeatureBuilderConfig {
    pub threshold_sideways: f64,
    pub threshold_trending: f64,
}

impl Default for ChoppinessIndexFeatureBuilderConfig {
    fn default() -> Self {
        return Self {
            threshold_sideways: CHOPPINESS_INDEX_THRESHOLD_SIDEWAYS,
            threshold_trending: CHOPPINESS_INDEX_THRESHOLD_TRENDING,
        };
    }
}

pub struct ChoppinessIndexFeatureBuilder {
    pub ctx: Context,
    pub inner: ChoppinessIndex,
    features: ChoppinessIndexFeatures,
    cross_over: CrossOverThreshold,
    cross_under: CrossUnderThreshold,
}

impl ChoppinessIndexFeatureBuilder {
    pub fn new(
        ctx: Context,
        config: ChoppinessIndexFeatureBuilderConfig,
        inner: ChoppinessIndex,
    ) -> Self {
        return Self {
            inner,
            ctx: ctx.clone(),
            features: ChoppinessIndexFeatures::default(),
            cross_over: CrossOverThreshold::new(ctx.clone(), config.threshold_sideways),
            cross_under: CrossUnderThreshold::new(ctx.clone(), config.threshold_trending),
        };
    }
}

impl IncrementalFeatureBuilder<ChoppinessIndexFeatures> for ChoppinessIndexFeatureBuilder {
    const NAMESPACE: &'static str = "ta::third_party::tradingview:::choppiness_index";
}

impl Incremental<(), ChoppinessIndexFeatures> for ChoppinessIndexFeatureBuilder {
    fn next(&mut self, _: ()) -> ChoppinessIndexFeatures {
        let value = self.inner.next(());

        self.features.value = rescale(
            value,
            CHOPPINESS_INDEX_MIN_VALUE,
            CHOPPINESS_INDEX_MAX_VALUE,
            -1.0,
            1.0,
        );

        let is_cross_over = self.cross_over.next(value);
        let is_cross_under = self.cross_under.next(value);

        self.features.signal = 0;

        if is_cross_over {
            self.features.signal = -1;
            self.features.trend = Some(Trend::Consolidation);
        } else if is_cross_under {
            self.features.signal = 1;
            self.features.trend = Some(Trend::Strong);
        }

        return self.features.clone();
    }
}

impl Incremental<(), Box<dyn Features>> for ChoppinessIndexFeatureBuilder {
    fn next(&mut self, _: ()) -> Box<dyn Features> {
        return Box::new(Incremental::<(), ChoppinessIndexFeatures>::next(self, ()));
    }
}
