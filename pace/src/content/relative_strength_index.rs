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

#[derive(Debug, Clone)]
pub struct RelativeStrengthIndexFeatures {
    pub value: f64,
    pub trend: Option<Trend>,
    pub signal: StrategySignal,
}

impl Default for RelativeStrengthIndexFeatures {
    fn default() -> Self {
        return Self {
            value: f64::NAN,
            trend: None,
            signal: StrategySignal::Hold,
        };
    }
}

impl Features for RelativeStrengthIndexFeatures {
    fn flatten(&self) -> HashMap<String, FeatureValue> {
        let mut map: HashMap<String, FeatureValue> = HashMap::new();

        map.insert("value".to_string(), self.value.into());
        map.insert(
            "trend".to_string(),
            self.trend.map(|x| x.into()).unwrap_or(FeatureValue::Empty),
        );
        map.insert("signal".to_string(), self.signal.into());

        return map;
    }
}

pub struct RelativeStrengthIndexFeatureBuilder {
    pub ctx: Context,
    pub inner: RelativeStrengthIndex,
    pub inner_strategy: RelativeStrengthIndexStrategy,
    features: RelativeStrengthIndexFeatures,
}

impl RelativeStrengthIndexFeatureBuilder {
    pub fn new(
        ctx: Context,
        inner: RelativeStrengthIndex,
        inner_strategy: RelativeStrengthIndexStrategy,
    ) -> Self {
        return Self {
            inner,
            inner_strategy,
            ctx,
            features: RelativeStrengthIndexFeatures::default(),
        };
    }
}

impl IncrementalFeatureBuilder<RelativeStrengthIndexFeatures>
    for RelativeStrengthIndexFeatureBuilder
{
    const NAMESPACE: &'static str = "ta::third_party::tradingview:::relative_strength_index";
}

impl Incremental<(), RelativeStrengthIndexFeatures> for RelativeStrengthIndexFeatureBuilder {
    fn next(&mut self, _: ()) -> RelativeStrengthIndexFeatures {
        let rsi = self.inner.next(());
        let signal = self.inner_strategy.next(rsi);

        self.features.value = rescale(
            rsi,
            RELATIVE_STRENGTH_INDEX_MIN_VALUE,
            RELATIVE_STRENGTH_INDEX_MAX_VALUE,
            -1.0,
            1.0,
        );
        self.features.signal = signal;

        if signal == StrategySignal::Long {
            self.features.trend = Some(Trend::Bullish);
        } else if signal == StrategySignal::Short {
            self.features.trend = Some(Trend::Bearish);
        }

        return self.features.clone();
    }
}

impl Incremental<(), Box<dyn Features>> for RelativeStrengthIndexFeatureBuilder {
    fn next(&mut self, _: ()) -> Box<dyn Features> {
        return Box::new(Incremental::<(), RelativeStrengthIndexFeatures>::next(
            self,
            (),
        ));
    }
}
