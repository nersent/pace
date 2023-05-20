use std::collections::HashMap;

use crate::{
    common::src::{AnyProcessor, AnySrc, Src, SrcKind},
    core::{
        context::Context,
        features::{FeatureValue, Features, IncrementalFeatureBuilder},
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
    },
};

pub struct AwesomeOscillatorConfig {
    pub short_src: AnySrc,
    pub long_src: AnySrc,
    pub short_ma: AnyProcessor,
    pub long_ma: AnyProcessor,
}

impl IncrementalDefault for AwesomeOscillatorConfig {
    fn default(ctx: Context) -> Self {
        Self {
            long_ma: Ma::new(ctx.clone(), MaKind::SMA, 34).to_box(),
            short_ma: Ma::new(ctx.clone(), MaKind::SMA, 5).to_box(),
            long_src: Src::new(ctx.clone(), SrcKind::HL2).to_box(),
            short_src: Src::new(ctx.clone(), SrcKind::HL2).to_box(),
        }
    }
}

/// Ported from https://www.tradingview.com/chart/?solution=43000501826
pub struct AwesomeOscillator {
    pub config: AwesomeOscillatorConfig,
    pub ctx: Context,
    prev_ao: f64,
}

impl AwesomeOscillator {
    pub fn new(ctx: Context, config: AwesomeOscillatorConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            config,
            prev_ao: f64::NAN,
        };
    }
}

impl Incremental<(), f64> for AwesomeOscillator {
    fn next(&mut self, _: ()) -> f64 {
        let short_ma_src = self.config.short_src.next(());
        let long_ma_src = self.config.long_src.next(());

        let short_ma = self.config.short_ma.next(short_ma_src);
        let long_ma = self.config.long_ma.next(long_ma_src);

        let ao = short_ma - long_ma;
        let osc = ao - self.prev_ao;

        self.prev_ao = ao;

        return osc;
    }
}

pub static AWESOME_OSCILLATOR_THRESHOLD_OVERSOLD: f64 = 0.0;
pub static AWESOME_OSCILLATOR_THRESHOLD_OVERBOUGHT: f64 = 0.0;

pub struct AwesomeOscillatorStrategyConfig {
    pub threshold_oversold: f64,
    pub threshold_overbought: f64,
}

impl Default for AwesomeOscillatorStrategyConfig {
    fn default() -> Self {
        return Self {
            threshold_oversold: AWESOME_OSCILLATOR_THRESHOLD_OVERSOLD,
            threshold_overbought: AWESOME_OSCILLATOR_THRESHOLD_OVERBOUGHT,
        };
    }
}

/// Custom Awesome Oscillator Strategy. May be incorrect.
pub struct AwesomeOscillatorStrategy {
    pub config: AwesomeOscillatorStrategyConfig,
    pub ctx: Context,
    cross_over: CrossOverThreshold,
    cross_under: CrossUnderThreshold,
}

impl AwesomeOscillatorStrategy {
    pub fn new(ctx: Context, config: AwesomeOscillatorStrategyConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            cross_over: CrossOverThreshold::new(ctx.clone(), config.threshold_oversold),
            cross_under: CrossUnderThreshold::new(ctx.clone(), config.threshold_overbought),
            config,
        };
    }
}

impl Incremental<f64, StrategySignal> for AwesomeOscillatorStrategy {
    fn next(&mut self, ao: f64) -> StrategySignal {
        let is_cross_over = self.cross_over.next(ao);
        let is_cross_under = self.cross_under.next(ao);

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
pub struct AwesomeOscillatorFeatures {
    pub value: f64,
    pub trend: Option<Trend>,
    pub signal: StrategySignal,
}

impl Default for AwesomeOscillatorFeatures {
    fn default() -> Self {
        return Self {
            value: f64::NAN,
            trend: None,
            signal: StrategySignal::Hold,
        };
    }
}

impl Features for AwesomeOscillatorFeatures {
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

pub struct AwesomeOscillatorFeatureBuilder {
    pub ctx: Context,
    pub inner: AwesomeOscillator,
    pub inner_strategy: AwesomeOscillatorStrategy,
    features: AwesomeOscillatorFeatures,
}

impl AwesomeOscillatorFeatureBuilder {
    pub fn new(
        ctx: Context,
        inner: AwesomeOscillator,
        inner_strategy: AwesomeOscillatorStrategy,
    ) -> Self {
        return Self {
            inner,
            inner_strategy,
            ctx,
            features: AwesomeOscillatorFeatures::default(),
        };
    }
}

impl IncrementalFeatureBuilder<AwesomeOscillatorFeatures> for AwesomeOscillatorFeatureBuilder {
    const NAMESPACE: &'static str = "ta::third_party::tradingview:::awesome_oscillator";
}

impl Incremental<(), AwesomeOscillatorFeatures> for AwesomeOscillatorFeatureBuilder {
    fn next(&mut self, _: ()) -> AwesomeOscillatorFeatures {
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

impl Incremental<(), Box<dyn Features>> for AwesomeOscillatorFeatureBuilder {
    fn next(&mut self, _: ()) -> Box<dyn Features> {
        return Box::new(Incremental::<(), AwesomeOscillatorFeatures>::next(self, ()));
    }
}
