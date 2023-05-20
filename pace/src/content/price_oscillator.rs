use std::collections::HashMap;

use crate::{
    common::src::{AnyProcessor, AnySrc, Src, SrcKind},
    core::{
        context::Context,
        features::{FeatureValue, Features, IncrementalFeatureBuilder},
        incremental::{Incremental, IncrementalDefault},
        trend::Trend,
    },
    strategy::trade::{StrategySignal, TradeDirection},
    ta::{
        cross::Cross,
        cross_over_threshold::CrossOverThreshold,
        cross_under_threshold::CrossUnderThreshold,
        highest_bars::HighestBars,
        lowest_bars::LowestBars,
        moving_average::{Ma, MaKind},
    },
    utils::float::Float64Utils,
};

pub struct PriceOscillatorConfig {
    pub src: AnySrc,
    pub short_ma: AnyProcessor,
    pub long_ma: AnyProcessor,
}

impl IncrementalDefault for PriceOscillatorConfig {
    fn default(ctx: Context) -> Self {
        Self {
            src: Src::new(ctx.clone(), SrcKind::Close).to_box(),
            short_ma: Ma::new(ctx.clone(), MaKind::SMA, 10).to_box(),
            long_ma: Ma::new(ctx.clone(), MaKind::SMA, 21).to_box(),
        }
    }
}

/// Ported from https://www.tradingview.com/chart/?solution=43000502346
pub struct PriceOscillator {
    pub config: PriceOscillatorConfig,
    pub ctx: Context,
}

impl PriceOscillator {
    pub fn new(ctx: Context, config: PriceOscillatorConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            config,
        };
    }
}

impl Incremental<(), f64> for PriceOscillator {
    fn next(&mut self, _: ()) -> f64 {
        let src = self.config.src.next(());

        let short_ma = self.config.short_ma.next(src);
        let long_ma = self.config.long_ma.next(src);
        let po = (short_ma - long_ma) / long_ma * 100.0;

        return po.normalize();
    }
}

pub static PRICE_OSCILLATOR_THRESHOLD_OVERSOLD: f64 = 0.0;
pub static PRICE_OSCILLATOR_THRESHOLD_OVERBOUGHT: f64 = 0.0;

pub struct PriceOscillatorStrategyConfig {
    pub threshold_oversold: f64,
    pub threshold_overbought: f64,
}

impl Default for PriceOscillatorStrategyConfig {
    fn default() -> Self {
        return Self {
            threshold_oversold: PRICE_OSCILLATOR_THRESHOLD_OVERSOLD,
            threshold_overbought: PRICE_OSCILLATOR_THRESHOLD_OVERBOUGHT,
        };
    }
}

/// Custom Price Oscillator Strategy. May be incorrect.
pub struct PriceOscillatorStrategy {
    pub config: PriceOscillatorStrategyConfig,
    pub ctx: Context,
    cross_over: CrossOverThreshold,
    cross_under: CrossUnderThreshold,
}

impl PriceOscillatorStrategy {
    pub fn new(ctx: Context, config: PriceOscillatorStrategyConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            cross_over: CrossOverThreshold::new(ctx.clone(), config.threshold_oversold),
            cross_under: CrossUnderThreshold::new(ctx.clone(), config.threshold_overbought),
            config,
        };
    }
}

impl Incremental<f64, StrategySignal> for PriceOscillatorStrategy {
    fn next(&mut self, po: f64) -> StrategySignal {
        let is_cross_over = self.cross_over.next(po);
        let is_cross_under = self.cross_under.next(po);

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
pub struct PriceOscillatorFeatures {
    pub value: f64,
    pub trend: Option<Trend>,
    pub signal: StrategySignal,
}

impl Default for PriceOscillatorFeatures {
    fn default() -> Self {
        return Self {
            value: f64::NAN,
            trend: None,
            signal: StrategySignal::Hold,
        };
    }
}

impl Features for PriceOscillatorFeatures {
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

pub struct PriceOscillatorFeatureBuilder {
    pub ctx: Context,
    pub inner: PriceOscillator,
    pub inner_strategy: PriceOscillatorStrategy,
    features: PriceOscillatorFeatures,
}

impl PriceOscillatorFeatureBuilder {
    pub fn new(
        ctx: Context,
        inner: PriceOscillator,
        inner_strategy: PriceOscillatorStrategy,
    ) -> Self {
        return Self {
            inner,
            inner_strategy,
            ctx,
            features: PriceOscillatorFeatures::default(),
        };
    }
}

impl IncrementalFeatureBuilder<PriceOscillatorFeatures> for PriceOscillatorFeatureBuilder {
    const NAMESPACE: &'static str = "ta::third_party::tradingview:::price_oscillator";
}

impl Incremental<(), PriceOscillatorFeatures> for PriceOscillatorFeatureBuilder {
    fn next(&mut self, _: ()) -> PriceOscillatorFeatures {
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

impl Incremental<(), Box<dyn Features>> for PriceOscillatorFeatureBuilder {
    fn next(&mut self, _: ()) -> Box<dyn Features> {
        return Box::new(Incremental::<(), PriceOscillatorFeatures>::next(self, ()));
    }
}
