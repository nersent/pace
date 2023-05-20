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
        cross::{Cross, CrossMode},
        cross_over_threshold::CrossOverThreshold,
        cross_under_threshold::CrossUnderThreshold,
        highest_bars::HighestBars,
        lowest_bars::LowestBars,
        moving_average::{Ma, MaKind},
        sum::Sum,
        symmetrically_weighted_moving_average::Swma,
    },
    utils::float::Float64Utils,
};

pub static RELATIVE_VIGOR_INDEX_MIN_VALUE: f64 = -1.0;
pub static RELATIVE_VIGOR_INDEX_MAX_VALUE: f64 = 1.0;

pub struct RelativeVigorIndexConfig {
    pub length: usize,
}

impl IncrementalDefault for RelativeVigorIndexConfig {
    fn default(ctx: Context) -> Self {
        Self { length: 10 }
    }
}

pub struct RelativeVigorIndexData {
    pub rvi: f64,
    pub sig: f64,
}

/// Ported from https://www.tradingview.com/chart/?solution=43000591593
pub struct RelativeVigorIndex {
    pub config: RelativeVigorIndexConfig,
    pub ctx: Context,
    swma_close_open: Swma,
    swma_high_low: Swma,
    sum_close_open: Sum,
    sum_high_low: Sum,
    swma_sig: Swma,
}

impl RelativeVigorIndex {
    pub fn new(ctx: Context, config: RelativeVigorIndexConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            swma_close_open: Swma::new(ctx.clone()),
            swma_high_low: Swma::new(ctx.clone()),
            sum_close_open: Sum::new(ctx.clone(), config.length),
            sum_high_low: Sum::new(ctx.clone(), config.length),
            swma_sig: Swma::new(ctx.clone()),
            config,
        };
    }
}

impl Incremental<(), RelativeVigorIndexData> for RelativeVigorIndex {
    fn next(&mut self, _: ()) -> RelativeVigorIndexData {
        let close = self.ctx.bar.close();
        let open = self.ctx.bar.open();
        let high = self.ctx.bar.high();
        let low = self.ctx.bar.low();

        let close_open_diff = close - open;
        let high_low_diff = high - low;

        let close_open_swma = self.swma_close_open.next(close_open_diff);
        let high_low_swma = self.swma_high_low.next(high_low_diff);

        let close_open_sum = self.sum_close_open.next(close_open_swma);
        let high_low_sum = self.sum_high_low.next(high_low_swma);

        let rvi = close_open_sum / high_low_sum;

        let sig = self.swma_sig.next(rvi);

        return RelativeVigorIndexData { rvi, sig };
    }
}

pub static RELATIVE_VIGOR_INDEX_THRESHOLD_OVERSOLD: f64 = 0.0;
pub static RELATIVE_VIGOR_INDEX_THRESHOLD_OVERBOUGHT: f64 = 0.0;

pub struct RelativeVigorIndexStrategyConfig {
    pub threshold_oversold: f64,
    pub threshold_overbought: f64,
}

impl Default for RelativeVigorIndexStrategyConfig {
    fn default() -> Self {
        return Self {
            threshold_oversold: RELATIVE_VIGOR_INDEX_THRESHOLD_OVERSOLD,
            threshold_overbought: RELATIVE_VIGOR_INDEX_THRESHOLD_OVERBOUGHT,
        };
    }
}

/// Custom Relative Vigor Index Strategy. May be incorrect.
pub struct RelativeVigorIndexStrategy {
    pub ctx: Context,
    pub config: RelativeVigorIndexStrategyConfig,
    // cross: Cross,
    cross_over: CrossOverThreshold,
    cross_under: CrossUnderThreshold,
}

impl RelativeVigorIndexStrategy {
    pub fn new(ctx: Context, config: RelativeVigorIndexStrategyConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            cross_over: CrossOverThreshold::new(ctx.clone(), config.threshold_oversold),
            cross_under: CrossUnderThreshold::new(ctx.clone(), config.threshold_overbought),
            config,
            // cross: Cross::new(ctx.clone()),
        };
    }
}

impl Incremental<&RelativeVigorIndexData, StrategySignal> for RelativeVigorIndexStrategy {
    fn next(&mut self, rvgi: &RelativeVigorIndexData) -> StrategySignal {
        let is_cross_over = self.cross_over.next(rvgi.rvi);
        let is_cross_under = self.cross_under.next(rvgi.rvi);

        if is_cross_over {
            return StrategySignal::Long;
        }
        if is_cross_under {
            return StrategySignal::Short;
        }
        return StrategySignal::Hold;
        // let rvi_s_cross = self.cross.next((rvgi.rvi, rvgi.sig));

        // if let Some(plus_minus_cross) = rvi_s_cross {
        //     if plus_minus_cross == CrossMode::Over {
        //         return StrategySignal::Long;
        //     } else if plus_minus_cross == CrossMode::Under {
        //         return StrategySignal::Short;
        //     }
        // }

        // return StrategySignal::Hold;
    }
}

#[derive(Debug, Clone)]
pub struct RelativeVigorIndexFeatures {
    pub value: f64,
    pub trend: Option<Trend>,
    pub signal: StrategySignal,
}

impl Default for RelativeVigorIndexFeatures {
    fn default() -> Self {
        return Self {
            value: f64::NAN,
            trend: None,
            signal: StrategySignal::Hold,
        };
    }
}

impl Features for RelativeVigorIndexFeatures {
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

pub struct RelativeVigorIndexFeatureBuilder {
    pub ctx: Context,
    pub inner: RelativeVigorIndex,
    pub inner_strategy: RelativeVigorIndexStrategy,
    features: RelativeVigorIndexFeatures,
}

impl RelativeVigorIndexFeatureBuilder {
    pub fn new(
        ctx: Context,
        inner: RelativeVigorIndex,
        inner_strategy: RelativeVigorIndexStrategy,
    ) -> Self {
        return Self {
            inner,
            inner_strategy,
            ctx,
            features: RelativeVigorIndexFeatures::default(),
        };
    }
}

impl IncrementalFeatureBuilder<RelativeVigorIndexFeatures> for RelativeVigorIndexFeatureBuilder {
    const NAMESPACE: &'static str = "ta::third_party::tradingview:::relative_vigor_index";
}

impl Incremental<(), RelativeVigorIndexFeatures> for RelativeVigorIndexFeatureBuilder {
    fn next(&mut self, _: ()) -> RelativeVigorIndexFeatures {
        let value = self.inner.next(());
        let signal = self.inner_strategy.next(&value);

        self.features.value = rescale(
            value.rvi,
            RELATIVE_VIGOR_INDEX_MIN_VALUE,
            RELATIVE_VIGOR_INDEX_MAX_VALUE,
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

impl Incremental<(), Box<dyn Features>> for RelativeVigorIndexFeatureBuilder {
    fn next(&mut self, _: ()) -> Box<dyn Features> {
        return Box::new(Incremental::<(), RelativeVigorIndexFeatures>::next(
            self,
            (),
        ));
    }
}
