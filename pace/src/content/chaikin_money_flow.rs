use std::collections::HashMap;

use crate::{
    common::src::{AnySrc, Src, SrcKind},
    core::{
        context::Context,
        features::{FeatureValue, Features, IncrementalFeatureBuilder},
        incremental::{Incremental, IncrementalDefault},
        trend::Trend,
    },
    pinescript::common::PineScriptFloat64,
    strategy::trade::{StrategySignal, TradeDirection},
    ta::{
        cross::Cross, cross_over_threshold::CrossOverThreshold,
        cross_under_threshold::CrossUnderThreshold, highest_bars::HighestBars,
        lowest_bars::LowestBars, sum::Sum,
    },
    utils::float::Float64Utils,
};

pub static CHAIKIN_MONEY_FLOW_MIN_VALUE: f64 = -1.0;
pub static CHAIKIN_MONEY_FLOW_MAX_VALUE: f64 = 1.0;

pub struct ChaikinMoneyFlowConfig {
    pub length: usize,
}

impl Default for ChaikinMoneyFlowConfig {
    fn default() -> Self {
        Self { length: 20 }
    }
}

pub struct ChaikinMoneyFlow {
    pub config: ChaikinMoneyFlowConfig,
    pub ctx: Context,
    volume_sum: Sum,
    ad_sum: Sum,
}

/// Ported from https://www.tradingview.com/chart/?solution=43000501974
impl ChaikinMoneyFlow {
    pub fn new(ctx: Context, config: ChaikinMoneyFlowConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            volume_sum: Sum::new(ctx.clone(), config.length),
            ad_sum: Sum::new(ctx.clone(), config.length),
            config,
        };
    }
}

impl Incremental<(), f64> for ChaikinMoneyFlow {
    fn next(&mut self, _: ()) -> f64 {
        let close = self.ctx.bar.close();
        let high = self.ctx.bar.high();
        let low = self.ctx.bar.low();
        let volume = self.ctx.bar.volume();

        let volume_sum = self.volume_sum.next(volume);

        let ad = (((2.0 * close - low - high) / (high - low)) * volume)
            .normalize()
            .ps_nz();

        let ad_sum = self.ad_sum.next(ad);

        let cmf = ad_sum / volume_sum;

        return cmf;
    }
}

pub static CHAIKIN_MONEY_FLOW_THRESHOLD_OVERSOLD: f64 = 0.0;
pub static CHAIKIN_MONEY_FLOW_THRESHOLD_OVERBOUGHT: f64 = 0.0;

pub struct ChaikinMoneyFlowStrategyConfig {
    pub threshold_oversold: f64,
    pub threshold_overbought: f64,
}

impl Default for ChaikinMoneyFlowStrategyConfig {
    fn default() -> Self {
        return Self {
            threshold_oversold: CHAIKIN_MONEY_FLOW_THRESHOLD_OVERSOLD,
            threshold_overbought: CHAIKIN_MONEY_FLOW_THRESHOLD_OVERBOUGHT,
        };
    }
}

/// Custom Chaikin Money Flow Strategy. May be incorrect.
pub struct ChaikinMoneyFlowStrategy {
    pub config: ChaikinMoneyFlowStrategyConfig,
    pub ctx: Context,
    cross_over: CrossOverThreshold,
    cross_under: CrossUnderThreshold,
}

impl ChaikinMoneyFlowStrategy {
    pub fn new(ctx: Context, config: ChaikinMoneyFlowStrategyConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            cross_over: CrossOverThreshold::new(ctx.clone(), config.threshold_oversold),
            cross_under: CrossUnderThreshold::new(ctx.clone(), config.threshold_overbought),
            config,
        };
    }
}

impl Incremental<f64, StrategySignal> for ChaikinMoneyFlowStrategy {
    fn next(&mut self, cmf: f64) -> StrategySignal {
        let is_cross_over = self.cross_over.next(cmf);
        let is_cross_under = self.cross_under.next(cmf);

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
pub struct ChaikinMoneyFlowFeatures {
    pub value: f64,
    pub trend: Option<Trend>,
    pub signal: StrategySignal,
}

impl Default for ChaikinMoneyFlowFeatures {
    fn default() -> Self {
        return Self {
            value: f64::NAN,
            trend: None,
            signal: StrategySignal::Hold,
        };
    }
}

impl Features for ChaikinMoneyFlowFeatures {
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

pub struct ChaikinMoneyFlowFeatureBuilder {
    pub ctx: Context,
    pub inner: ChaikinMoneyFlow,
    pub inner_strategy: ChaikinMoneyFlowStrategy,
    features: ChaikinMoneyFlowFeatures,
}

impl ChaikinMoneyFlowFeatureBuilder {
    pub fn new(
        ctx: Context,
        inner: ChaikinMoneyFlow,
        inner_strategy: ChaikinMoneyFlowStrategy,
    ) -> Self {
        return Self {
            inner,
            inner_strategy,
            ctx,
            features: ChaikinMoneyFlowFeatures::default(),
        };
    }
}

impl IncrementalFeatureBuilder<ChaikinMoneyFlowFeatures> for ChaikinMoneyFlowFeatureBuilder {
    const NAMESPACE: &'static str = "ta::third_party::tradingview:::chaikin_money_flow";
}

impl Incremental<(), ChaikinMoneyFlowFeatures> for ChaikinMoneyFlowFeatureBuilder {
    fn next(&mut self, _: ()) -> ChaikinMoneyFlowFeatures {
        let value = self.inner.next(());
        let signal = self.inner_strategy.next(value);

        self.features.value = value;
        self.features.signal = signal;

        if signal == StrategySignal::Long {
            self.features.trend = Some(Trend::Bullish);
        } else if signal == StrategySignal::Short {
            self.features.trend = Some(Trend::Bearish);
        }

        return self.features.clone();
    }
}

impl Incremental<(), Box<dyn Features>> for ChaikinMoneyFlowFeatureBuilder {
    fn next(&mut self, _: ()) -> Box<dyn Features> {
        return Box::new(Incremental::<(), ChaikinMoneyFlowFeatures>::next(self, ()));
    }
}
