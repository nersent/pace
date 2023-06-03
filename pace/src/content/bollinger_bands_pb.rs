use std::collections::HashMap;

use crate::{
    common::src::{AnySrc, Src, SrcKind},
    core::{
        context::Context,
        features::{FeatureValue, Features, IncrementalFeatureBuilder},
        incremental::{Incremental, IncrementalDefault},
        trend::Trend,
    },
    strategy::trade::{StrategySignal, TradeDirection},
    ta::{
        cross::Cross, cross_over_threshold::CrossOverThreshold,
        cross_under_threshold::CrossUnderThreshold, highest_bars::HighestBars,
        lowest_bars::LowestBars, simple_moving_average::Sma, stdev::Stdev,
    },
    utils::float::Float64Utils,
};

pub static BOLLINGER_BANDS_PERCENT_B_MULT: f64 = 2.0;

pub struct BollingerBandsPercentBConfig {
    pub length: usize,
    pub src: AnySrc,
    pub mult: f64,
}

impl IncrementalDefault for BollingerBandsPercentBConfig {
    fn default(ctx: Context) -> Self {
        Self {
            length: 20,
            src: Box::new(Src::new(ctx.clone(), SrcKind::Close)),
            mult: BOLLINGER_BANDS_PERCENT_B_MULT,
        }
    }
}

/// Ported from https://www.tradingview.com/chart/?solution=43000501971
pub struct BollingerBandsPercentB {
    pub config: BollingerBandsPercentBConfig,
    pub ctx: Context,
    basis: Sma,
    stdev: Stdev,
}

impl BollingerBandsPercentB {
    pub fn new(ctx: Context, config: BollingerBandsPercentBConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            basis: Sma::new(ctx.clone(), config.length),
            stdev: Stdev::new(ctx.clone(), config.length, true),
            config,
        };
    }
}

impl Incremental<(), f64> for BollingerBandsPercentB {
    fn next(&mut self, _: ()) -> f64 {
        let src = self.config.src.next(());
        let basis = self.basis.next(src);
        let dev = self.stdev.next(src);

        let dev = dev * self.config.mult;
        let upper = basis + dev;
        let lower = basis - dev;
        let upper_lower_diff = upper - lower;

        if upper_lower_diff.is_zero() {
            return f64::NAN;
        }

        let bbr = (src - lower) / upper_lower_diff;

        return bbr;
    }
}

pub static BOLLINGER_BANDS_PERCENT_B_THRESHOLD_OVERSOLD: f64 = 0.0;
pub static BOLLINGER_BANDS_PERCENT_B_THRESHOLD_OVERBOUGHT: f64 = 1.0;

pub struct BollingerBandsPercentBStrategyConfig {
    pub threshold_oversold: f64,
    pub threshold_overbought: f64,
}

impl Default for BollingerBandsPercentBStrategyConfig {
    fn default() -> Self {
        return Self {
            threshold_overbought: BOLLINGER_BANDS_PERCENT_B_THRESHOLD_OVERBOUGHT,
            threshold_oversold: BOLLINGER_BANDS_PERCENT_B_THRESHOLD_OVERSOLD,
        };
    }
}

/// Custom Bollinger Bands %B Strategy. May be incorrect.
///
/// Ported from https://www.tradingview.com/chart/?solution=43000589104
pub struct BollingerBandsPercentBStrategy {
    pub config: BollingerBandsPercentBStrategyConfig,
    pub ctx: Context,
    cross_over: CrossOverThreshold,
    cross_under: CrossUnderThreshold,
}

impl BollingerBandsPercentBStrategy {
    pub fn new(ctx: Context, config: BollingerBandsPercentBStrategyConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            cross_over: CrossOverThreshold::new(ctx.clone(), config.threshold_oversold),
            cross_under: CrossUnderThreshold::new(ctx.clone(), config.threshold_overbought),
            config,
        };
    }
}

impl Incremental<f64, StrategySignal> for BollingerBandsPercentBStrategy {
    fn next(&mut self, bbpb: f64) -> StrategySignal {
        let is_cross_over = self.cross_over.next(bbpb);
        let is_cross_under = self.cross_under.next(bbpb);

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
pub struct BollingerBandsPercentBFeatures {
    pub value: f64,
    pub trend: Option<Trend>,
    pub signal: StrategySignal,
}

impl Default for BollingerBandsPercentBFeatures {
    fn default() -> Self {
        return Self {
            value: f64::NAN,
            trend: None,
            signal: StrategySignal::Hold,
        };
    }
}

impl Features for BollingerBandsPercentBFeatures {
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

pub struct BollingerBandsPercentBFeatureBuilder {
    pub ctx: Context,
    pub inner: BollingerBandsPercentB,
    pub inner_strategy: BollingerBandsPercentBStrategy,
    features: BollingerBandsPercentBFeatures,
}

impl BollingerBandsPercentBFeatureBuilder {
    pub fn new(
        ctx: Context,
        inner: BollingerBandsPercentB,
        inner_strategy: BollingerBandsPercentBStrategy,
    ) -> Self {
        return Self {
            inner,
            inner_strategy,
            ctx,
            features: BollingerBandsPercentBFeatures::default(),
        };
    }
}

// impl IncrementalFeatureBuilder<BollingerBandsPercentBFeatures>
//     for BollingerBandsPercentBFeatureBuilder
// {
//     const NAMESPACE: &'static str = "ta::third_party::tradingview:::bollinger_bands_pb";
// }

impl Incremental<(), BollingerBandsPercentBFeatures> for BollingerBandsPercentBFeatureBuilder {
    fn next(&mut self, _: ()) -> BollingerBandsPercentBFeatures {
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

impl Incremental<(), Box<dyn Features>> for BollingerBandsPercentBFeatureBuilder {
    fn next(&mut self, _: ()) -> Box<dyn Features> {
        return Box::new(Incremental::<(), BollingerBandsPercentBFeatures>::next(
            self,
            (),
        ));
    }
}
