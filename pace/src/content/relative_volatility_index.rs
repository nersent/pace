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
    strategy::trade::{StrategySignal, TradeDirection},
    ta::{
        cross::Cross,
        cross_over_threshold::CrossOverThreshold,
        cross_under_threshold::CrossUnderThreshold,
        exponential_moving_average::Ema,
        highest_bars::HighestBars,
        lowest_bars::LowestBars,
        moving_average::{Ma, MaKind},
        stdev::Stdev,
    },
};

pub static RELATIVE_VOLATILITY_INDEX_MIN_VALUE: f64 = 0.0;
pub static RELATIVE_VOLATILITY_INDEX_MAX_VALUE: f64 = 100.0;

pub struct RelativeVolatilityIndexConfig {
    pub length: usize,
    pub ma_length: usize,
    pub src: AnySrc,
}

impl IncrementalDefault for RelativeVolatilityIndexConfig {
    fn default(ctx: Context) -> Self {
        Self {
            length: 10,
            ma_length: 14,
            src: Src::new(ctx.clone(), SrcKind::Close).to_box(),
        }
    }
}

/// Ported from https://www.tradingview.com/chart/?solution=43000594684
pub struct RelativeVolatilityIndex {
    pub config: RelativeVolatilityIndexConfig,
    pub ctx: Context,
    stdev: Stdev,
    upper_ema: Ema,
    lower_ema: Ema,
    prev_src: f64,
}

impl RelativeVolatilityIndex {
    pub fn new(ctx: Context, config: RelativeVolatilityIndexConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            stdev: Stdev::new(ctx.clone(), config.length, true),
            upper_ema: Ema::new(ctx.clone(), config.ma_length),
            lower_ema: Ema::new(ctx.clone(), config.ma_length),
            config,
            prev_src: f64::NAN,
        };
    }
}

impl Incremental<(), f64> for RelativeVolatilityIndex {
    fn next(&mut self, _: ()) -> f64 {
        let src = self.config.src.next(());
        let stdev = self.stdev.next(src);
        let change = src - self.prev_src;

        let (upper, lower) = if !change.is_nan() {
            let upper = if change <= 0.0 { 0.0 } else { stdev };
            let lower = if change > 0.0 { 0.0 } else { stdev };
            (upper, lower)
        } else {
            (f64::NAN, f64::NAN)
        };

        let upper = self.upper_ema.next(upper);
        let lower = self.lower_ema.next(lower);

        let rvi = if !upper.is_nan() && !lower.is_nan() && upper != lower {
            upper / (upper + lower) * 100.0
        } else {
            f64::NAN
        };

        self.prev_src = src;

        return rvi;
    }
}

pub static RELATIVE_VOLATILITY_INDEX_THRESHOLD_OVERSOLD: f64 = 20.0;
pub static RELATIVE_VOLATILITY_INDEX_THRESHOLD_OVERBOUGHT: f64 = 80.0;

pub struct RelativeVolatilityIndexStrategyConfig {
    pub threshold_oversold: f64,
    pub threshold_overbought: f64,
}

impl Default for RelativeVolatilityIndexStrategyConfig {
    fn default() -> Self {
        return Self {
            threshold_oversold: RELATIVE_VOLATILITY_INDEX_THRESHOLD_OVERSOLD,
            threshold_overbought: RELATIVE_VOLATILITY_INDEX_THRESHOLD_OVERBOUGHT,
        };
    }
}

/// Custom Relative Volatility Index Strategy. May be incorrect.
pub struct RelativeVolatilityIndexStrategy {
    pub config: RelativeVolatilityIndexStrategyConfig,
    pub ctx: Context,
    cross_over: CrossOverThreshold,
    cross_under: CrossUnderThreshold,
}

impl RelativeVolatilityIndexStrategy {
    pub fn new(ctx: Context, config: RelativeVolatilityIndexStrategyConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            cross_over: CrossOverThreshold::new(ctx.clone(), config.threshold_oversold),
            cross_under: CrossUnderThreshold::new(ctx.clone(), config.threshold_overbought),
            config,
        };
    }
}

impl Incremental<f64, StrategySignal> for RelativeVolatilityIndexStrategy {
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
pub struct RelativeVolatilityIndexFeatures {
    pub value: f64,
    pub trend: Option<Trend>,
    pub signal: StrategySignal,
}

impl Default for RelativeVolatilityIndexFeatures {
    fn default() -> Self {
        return Self {
            value: f64::NAN,
            trend: None,
            signal: StrategySignal::Hold,
        };
    }
}

impl Features for RelativeVolatilityIndexFeatures {
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

pub struct RelativeVolatilityIndexFeatureBuilder {
    pub ctx: Context,
    pub inner: RelativeVolatilityIndex,
    pub inner_strategy: RelativeVolatilityIndexStrategy,
    features: RelativeVolatilityIndexFeatures,
}

impl RelativeVolatilityIndexFeatureBuilder {
    pub fn new(
        ctx: Context,
        inner: RelativeVolatilityIndex,
        inner_strategy: RelativeVolatilityIndexStrategy,
    ) -> Self {
        return Self {
            inner,
            inner_strategy,
            ctx,
            features: RelativeVolatilityIndexFeatures::default(),
        };
    }
}

impl IncrementalFeatureBuilder<RelativeVolatilityIndexFeatures>
    for RelativeVolatilityIndexFeatureBuilder
{
    const NAMESPACE: &'static str = "ta::third_party::tradingview:::relative_volatility_index";
}

impl Incremental<(), RelativeVolatilityIndexFeatures> for RelativeVolatilityIndexFeatureBuilder {
    fn next(&mut self, _: ()) -> RelativeVolatilityIndexFeatures {
        let value = self.inner.next(());
        let signal = self.inner_strategy.next(value);

        self.features.value = rescale(
            value,
            RELATIVE_VOLATILITY_INDEX_MIN_VALUE,
            RELATIVE_VOLATILITY_INDEX_MAX_VALUE,
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

impl Incremental<(), Box<dyn Features>> for RelativeVolatilityIndexFeatureBuilder {
    fn next(&mut self, _: ()) -> Box<dyn Features> {
        return Box::new(Incremental::<(), RelativeVolatilityIndexFeatures>::next(
            self,
            (),
        ));
    }
}
