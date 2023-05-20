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
        cross::Cross, cross_over_threshold::CrossOverThreshold,
        cross_under_threshold::CrossUnderThreshold, dev::Dev, highest_bars::HighestBars,
        lowest_bars::LowestBars, simple_moving_average::Sma,
    },
    utils::float::Float64Utils,
};

pub struct CommodityChannelIndexConfig {
    pub length: usize,
    pub src: AnySrc,
}

impl IncrementalDefault for CommodityChannelIndexConfig {
    fn default(ctx: Context) -> Self {
        Self {
            length: 20,
            src: Src::new(ctx.clone(), SrcKind::HLC3).to_box(),
        }
    }
}

/// Ported from https://www.tradingview.com/chart/?solution=43000502001
pub struct CommodityChannelIndex {
    pub config: CommodityChannelIndexConfig,
    pub ctx: Context,
    sma: Sma,
    dev: Dev,
}

impl CommodityChannelIndex {
    pub fn new(ctx: Context, config: CommodityChannelIndexConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            sma: Sma::new(ctx.clone(), config.length),
            dev: Dev::new(ctx.clone(), config.length),
            config,
        };
    }
}

impl Incremental<(), f64> for CommodityChannelIndex {
    fn next(&mut self, _: ()) -> f64 {
        let src = self.config.src.next(());
        let ma = self.sma.next(src);
        let dev = self.dev.next(src);

        let cci = (src - ma) / (dev * 0.015);

        return cci;
    }
}

pub static COMMODITY_CHANNEL_INDEX_THRESHOLD_OVERSOLD: f64 = -200.0;
pub static COMMODITY_CHANNEL_INDEX_THRESHOLD_OVERBOUGHT: f64 = 200.0;

pub struct CommodityChannelIndexStrategyConfig {
    pub threshold_oversold: f64,
    pub threshold_overbought: f64,
}

impl Default for CommodityChannelIndexStrategyConfig {
    fn default() -> Self {
        return Self {
            threshold_oversold: COMMODITY_CHANNEL_INDEX_THRESHOLD_OVERSOLD,
            threshold_overbought: COMMODITY_CHANNEL_INDEX_THRESHOLD_OVERBOUGHT,
        };
    }
}

/// Custom Commodity Channel Index Strategy. May be incorrect.
pub struct CommodityChannelIndexStrategy {
    pub config: CommodityChannelIndexStrategyConfig,
    pub ctx: Context,
    cross_over: CrossOverThreshold,
    cross_under: CrossUnderThreshold,
}

impl CommodityChannelIndexStrategy {
    pub fn new(ctx: Context, config: CommodityChannelIndexStrategyConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            cross_over: CrossOverThreshold::new(ctx.clone(), config.threshold_oversold),
            cross_under: CrossUnderThreshold::new(ctx.clone(), config.threshold_overbought),
            config,
        };
    }
}

impl Incremental<f64, StrategySignal> for CommodityChannelIndexStrategy {
    fn next(&mut self, cci: f64) -> StrategySignal {
        let is_cross_over = self.cross_over.next(cci);
        let is_cross_under = self.cross_under.next(cci);

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
pub struct CommodityChannelIndexFeatures {
    pub value: f64,
    pub trend: Option<Trend>,
    pub signal: StrategySignal,
}

impl Default for CommodityChannelIndexFeatures {
    fn default() -> Self {
        return Self {
            value: f64::NAN,
            trend: None,
            signal: StrategySignal::Hold,
        };
    }
}

impl Features for CommodityChannelIndexFeatures {
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

pub struct CommodityChannelIndexFeatureBuilder {
    pub ctx: Context,
    pub inner: CommodityChannelIndex,
    pub inner_strategy: CommodityChannelIndexStrategy,
    features: CommodityChannelIndexFeatures,
}

impl CommodityChannelIndexFeatureBuilder {
    pub fn new(
        ctx: Context,
        inner: CommodityChannelIndex,
        inner_strategy: CommodityChannelIndexStrategy,
    ) -> Self {
        return Self {
            inner,
            inner_strategy,
            ctx,
            features: CommodityChannelIndexFeatures::default(),
        };
    }
}

impl IncrementalFeatureBuilder<CommodityChannelIndexFeatures>
    for CommodityChannelIndexFeatureBuilder
{
    const NAMESPACE: &'static str = "ta::third_party::tradingview:::commodity_channel_index";
}

impl Incremental<(), CommodityChannelIndexFeatures> for CommodityChannelIndexFeatureBuilder {
    fn next(&mut self, _: ()) -> CommodityChannelIndexFeatures {
        let cci = self.inner.next(());
        let signal = self.inner_strategy.next(cci);

        self.features.value = cci;
        self.features.signal = signal;

        if signal == StrategySignal::Long {
            self.features.trend = Some(Trend::Bullish);
        } else if signal == StrategySignal::Short {
            self.features.trend = Some(Trend::Bearish);
        }

        return self.features.clone();
    }
}

impl Incremental<(), Box<dyn Features>> for CommodityChannelIndexFeatureBuilder {
    fn next(&mut self, _: ()) -> Box<dyn Features> {
        return Box::new(Incremental::<(), CommodityChannelIndexFeatures>::next(
            self,
            (),
        ));
    }
}
