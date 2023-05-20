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
        highest_bars::HighestBars,
        lowest_bars::LowestBars,
        moving_average::{Ma, MaKind},
        relative_strength_index::Rsi,
        simple_moving_average::Sma,
        stoch::Stoch,
    },
};

pub static STOCH_RELATIVE_STRENGTH_INDEX_MIN_VALUE: f64 = 0.0;
pub static STOCH_RELATIVE_STRENGTH_INDEX_MAX_VALUE: f64 = 100.0;

pub struct StochRelativeStrengthIndexConfig {
    pub length_rsi: usize,
    pub length_stoch: usize,
    pub smooth_k: usize,
    pub smooth_d: usize,
    pub src: AnySrc,
}

impl IncrementalDefault for StochRelativeStrengthIndexConfig {
    fn default(ctx: Context) -> Self {
        return Self {
            length_rsi: 14,
            length_stoch: 14,
            smooth_k: 3,
            smooth_d: 3,
            src: Src::new(ctx.clone(), SrcKind::Close).to_box(),
        };
    }
}

pub struct StochRelativeStrengthIndexData {
    pub k: f64,
    pub d: f64,
}
/// Ported from https://www.tradingview.com/chart/?solution=43000502333
pub struct StochRelativeStrengthIndex {
    pub config: StochRelativeStrengthIndexConfig,
    pub ctx: Context,
    rsi: Rsi,
    k_stoch: Stoch,
    k_sma: Sma,
    d_sma: Sma,
}

impl StochRelativeStrengthIndex {
    pub fn new(ctx: Context, config: StochRelativeStrengthIndexConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            rsi: Rsi::new(ctx.clone(), config.length_rsi),
            k_stoch: Stoch::new(ctx.clone(), config.length_stoch),
            k_sma: Sma::new(ctx.clone(), config.smooth_k),
            d_sma: Sma::new(ctx.clone(), config.smooth_d),
            config,
        };
    }
}

impl Incremental<(), StochRelativeStrengthIndexData> for StochRelativeStrengthIndex {
    fn next(&mut self, _: ()) -> StochRelativeStrengthIndexData {
        let src = self.config.src.next(());
        let rsi = self.rsi.next(src);

        let k_stoch = self.k_stoch.next((rsi, rsi, rsi));
        let k_sma = self.k_sma.next(k_stoch);
        let d_sma = self.d_sma.next(k_sma);

        return StochRelativeStrengthIndexData { k: k_sma, d: d_sma };
    }
}

pub static STOCH_RELATIVE_STRENGTH_INDEX_THRESHOLD_OVERSOLD: f64 = 20.0;
pub static STOCH_RELATIVE_STRENGTH_INDEX_THRESHOLD_OVERBOUGHT: f64 = 80.0;

pub struct StochRelativeStrengthIndexStrategyConfig {
    pub threshold_oversold: f64,
    pub threshold_overbought: f64,
}

impl Default for StochRelativeStrengthIndexStrategyConfig {
    fn default() -> Self {
        return Self {
            threshold_oversold: STOCH_RELATIVE_STRENGTH_INDEX_THRESHOLD_OVERSOLD,
            threshold_overbought: STOCH_RELATIVE_STRENGTH_INDEX_THRESHOLD_OVERBOUGHT,
        };
    }
}

/// Custom Stochastic Relative Strength Index Strategy. May be incorrect.
pub struct StochRelativeStrengthIndexStrategy {
    pub config: StochRelativeStrengthIndexStrategyConfig,
    pub ctx: Context,
    cross_overbought: CrossOverThreshold,
    cross_oversold: CrossUnderThreshold,
}

impl StochRelativeStrengthIndexStrategy {
    pub fn new(ctx: Context, config: StochRelativeStrengthIndexStrategyConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            cross_overbought: CrossOverThreshold::new(ctx.clone(), config.threshold_oversold),
            cross_oversold: CrossUnderThreshold::new(ctx.clone(), config.threshold_overbought),
            config,
        };
    }
}

impl Incremental<&StochRelativeStrengthIndexData, StrategySignal>
    for StochRelativeStrengthIndexStrategy
{
    fn next(&mut self, stoch_rsi: &StochRelativeStrengthIndexData) -> StrategySignal {
        let is_cross_over = self.cross_overbought.next(stoch_rsi.k);
        let is_cross_under = self.cross_oversold.next(stoch_rsi.k);

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
pub struct StochRelativeStrengthIndexFeatures {
    pub k: f64,
    pub trend: Option<Trend>,
    pub signal: StrategySignal,
}

impl Default for StochRelativeStrengthIndexFeatures {
    fn default() -> Self {
        return Self {
            k: f64::NAN,
            trend: None,
            signal: StrategySignal::Hold,
        };
    }
}

impl Features for StochRelativeStrengthIndexFeatures {
    fn flatten(&self) -> HashMap<String, FeatureValue> {
        let mut map: HashMap<String, FeatureValue> = HashMap::new();

        map.insert("k".to_string(), self.k.into());
        map.insert(
            "trend".to_string(),
            self.trend.map(|x| x.into()).unwrap_or(FeatureValue::Empty),
        );
        map.insert("signal".to_string(), self.signal.into());

        return map;
    }
}

pub struct StochRelativeStrengthIndexFeatureBuilder {
    pub ctx: Context,
    pub inner: StochRelativeStrengthIndex,
    pub inner_strategy: StochRelativeStrengthIndexStrategy,
    features: StochRelativeStrengthIndexFeatures,
}

impl StochRelativeStrengthIndexFeatureBuilder {
    pub fn new(
        ctx: Context,
        inner: StochRelativeStrengthIndex,
        inner_strategy: StochRelativeStrengthIndexStrategy,
    ) -> Self {
        return Self {
            inner,
            inner_strategy,
            ctx,
            features: StochRelativeStrengthIndexFeatures::default(),
        };
    }
}

impl IncrementalFeatureBuilder<StochRelativeStrengthIndexFeatures>
    for StochRelativeStrengthIndexFeatureBuilder
{
    const NAMESPACE: &'static str = "ta::third_party::tradingview:::stoch_relative_strength_index";
}

impl Incremental<(), StochRelativeStrengthIndexFeatures>
    for StochRelativeStrengthIndexFeatureBuilder
{
    fn next(&mut self, _: ()) -> StochRelativeStrengthIndexFeatures {
        let value = self.inner.next(());
        let signal = self.inner_strategy.next(&value);

        self.features.k = rescale(
            value.k,
            STOCH_RELATIVE_STRENGTH_INDEX_MIN_VALUE,
            STOCH_RELATIVE_STRENGTH_INDEX_MAX_VALUE,
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

impl Incremental<(), Box<dyn Features>> for StochRelativeStrengthIndexFeatureBuilder {
    fn next(&mut self, _: ()) -> Box<dyn Features> {
        return Box::new(Incremental::<(), StochRelativeStrengthIndexFeatures>::next(
            self,
            (),
        ));
    }
}
