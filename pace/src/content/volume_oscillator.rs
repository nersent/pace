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

pub static VOLUME_OSCILLATOR_MIN_VALUE: f64 = -100.0;
pub static VOLUME_OSCILLATOR_MAX_VALUE: f64 = 100.0;

pub struct VolumeOscillatorConfig {
    pub short_ma: AnyProcessor,
    pub long_ma: AnyProcessor,
}

impl IncrementalDefault for VolumeOscillatorConfig {
    fn default(ctx: Context) -> Self {
        Self {
            short_ma: Ma::new(ctx.clone(), MaKind::EMA, 5).to_box(),
            long_ma: Ma::new(ctx.clone(), MaKind::EMA, 10).to_box(),
        }
    }
}

/// Ported from https://www.tradingview.com/chart/?solution=43000591350
pub struct VolumeOscillator {
    pub config: VolumeOscillatorConfig,
    pub ctx: Context,
}

impl VolumeOscillator {
    pub fn new(ctx: Context, config: VolumeOscillatorConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            config,
        };
    }
}

impl Incremental<(), f64> for VolumeOscillator {
    fn next(&mut self, _: ()) -> f64 {
        let volume = self.ctx.bar.volume();

        let short_ma = self.config.short_ma.next(volume);
        let long_ma = self.config.long_ma.next(volume);

        let osc = ((short_ma - long_ma) / long_ma) * 100.0;

        return osc;
    }
}

pub static VOLUME_OSCILLATOR_THRESHOLD_OVERSOLD: f64 = 0.0;
pub static VOLUME_OSCILLATOR_THRESHOLD_OVERBOUGHT: f64 = 0.0;

pub struct VolumeOscillatorStrategyConfig {
    pub threshold_oversold: f64,
    pub threshold_overbought: f64,
}

impl Default for VolumeOscillatorStrategyConfig {
    fn default() -> Self {
        return Self {
            threshold_oversold: VOLUME_OSCILLATOR_THRESHOLD_OVERSOLD,
            threshold_overbought: VOLUME_OSCILLATOR_THRESHOLD_OVERBOUGHT,
        };
    }
}

/// Custom Volume Oscillator Strategy. May be incorrect.
pub struct VolumeOscillatorStrategy {
    pub ctx: Context,
    cross_over: CrossOverThreshold,
    cross_under: CrossUnderThreshold,
}

impl VolumeOscillatorStrategy {
    pub fn new(ctx: Context, config: VolumeOscillatorStrategyConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            cross_over: CrossOverThreshold::new(ctx.clone(), config.threshold_oversold),
            cross_under: CrossUnderThreshold::new(ctx.clone(), config.threshold_overbought),
        };
    }
}

impl Incremental<f64, StrategySignal> for VolumeOscillatorStrategy {
    fn next(&mut self, vo: f64) -> StrategySignal {
        let is_cross_over = self.cross_over.next(vo);
        let is_cross_under = self.cross_under.next(vo);

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
pub struct VolumeOscillatorFeatures {
    pub value: f64,
    pub trend: Option<Trend>,
    pub signal: StrategySignal,
}

impl Default for VolumeOscillatorFeatures {
    fn default() -> Self {
        return Self {
            value: f64::NAN,
            trend: None,
            signal: StrategySignal::Hold,
        };
    }
}

impl Features for VolumeOscillatorFeatures {
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

pub struct VolumeOscillatorFeatureBuilder {
    pub ctx: Context,
    pub inner: VolumeOscillator,
    pub inner_strategy: VolumeOscillatorStrategy,
    features: VolumeOscillatorFeatures,
}

impl VolumeOscillatorFeatureBuilder {
    pub fn new(
        ctx: Context,
        inner: VolumeOscillator,
        inner_strategy: VolumeOscillatorStrategy,
    ) -> Self {
        return Self {
            inner,
            inner_strategy,
            ctx,
            features: VolumeOscillatorFeatures::default(),
        };
    }
}

impl IncrementalFeatureBuilder<VolumeOscillatorFeatures> for VolumeOscillatorFeatureBuilder {
    const NAMESPACE: &'static str = "ta::third_party::tradingview:::volume_oscillator";
}

impl Incremental<(), VolumeOscillatorFeatures> for VolumeOscillatorFeatureBuilder {
    fn next(&mut self, _: ()) -> VolumeOscillatorFeatures {
        let value = self.inner.next(());
        let signal = self.inner_strategy.next(value);

        self.features.value = rescale(
            value,
            VOLUME_OSCILLATOR_MIN_VALUE,
            VOLUME_OSCILLATOR_MAX_VALUE,
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

impl Incremental<(), Box<dyn Features>> for VolumeOscillatorFeatureBuilder {
    fn next(&mut self, _: ()) -> Box<dyn Features> {
        return Box::new(Incremental::<(), VolumeOscillatorFeatures>::next(self, ()));
    }
}
