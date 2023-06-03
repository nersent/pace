use std::collections::HashMap;

use crate::{
    common::src::{AnySrc, Src, SrcKind},
    core::{
        context::Context,
        features::{FeatureValue, Features, IncrementalFeatureBuilder},
        incremental::{Incremental, IncrementalDefault},
    },
    strategy::trade::TradeDirection,
    ta::{
        cross::Cross, cross_over_threshold::CrossOverThreshold,
        cross_under_threshold::CrossUnderThreshold, highest_bars::HighestBars,
        lowest_bars::LowestBars, simple_moving_average::Sma, stdev::Stdev,
    },
    utils::float::Float64Utils,
};

pub static BBW_MULT: f64 = 2.0;

pub struct BollingerBandsWidthConfig {
    pub length: usize,
    pub src: AnySrc,
    pub mult: f64,
}

impl IncrementalDefault for BollingerBandsWidthConfig {
    fn default(ctx: Context) -> Self {
        Self {
            length: 20,
            src: Box::new(Src::new(ctx.clone(), SrcKind::Close)),
            mult: BBW_MULT,
        }
    }
}

/// Ported from https://www.tradingview.com/chart/?solution=43000501972
pub struct BollingerBandsWidth {
    pub config: BollingerBandsWidthConfig,
    pub ctx: Context,
    basis: Sma,
    stdev: Stdev,
}

impl BollingerBandsWidth {
    pub fn new(ctx: Context, config: BollingerBandsWidthConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            basis: Sma::new(ctx.clone(), config.length),
            stdev: Stdev::new(ctx.clone(), config.length, true),
            config,
        };
    }
}

impl Incremental<(), f64> for BollingerBandsWidth {
    fn next(&mut self, _: ()) -> f64 {
        let src = self.config.src.next(());
        let basis = self.basis.next(src);
        let dev = self.stdev.next(src);

        if basis.is_zero() {
            return f64::NAN;
        }

        let dev = dev * self.config.mult;
        let upper = basis + dev;
        let lower = basis - dev;
        let bbw = (upper - lower) / basis;

        return bbw;
    }
}

#[derive(Debug, Clone)]
pub struct BollingerBandsWidthFeatures {
    pub value: f64,
}

impl Default for BollingerBandsWidthFeatures {
    fn default() -> Self {
        return Self { value: f64::NAN };
    }
}

impl Features for BollingerBandsWidthFeatures {
    fn flatten(&self) -> HashMap<String, FeatureValue> {
        let mut map: HashMap<String, FeatureValue> = HashMap::new();

        map.insert("value".to_string(), self.value.into());

        return map;
    }
}

pub struct BollingerBandsWidthFeatureBuilder {
    pub ctx: Context,
    pub inner: BollingerBandsWidth,
    features: BollingerBandsWidthFeatures,
}

impl BollingerBandsWidthFeatureBuilder {
    pub fn new(ctx: Context, inner: BollingerBandsWidth) -> Self {
        return Self {
            inner,
            ctx,
            features: BollingerBandsWidthFeatures::default(),
        };
    }
}

// impl IncrementalFeatureBuilder<BollingerBandsWidthFeatures> for BollingerBandsWidthFeatureBuilder {
//     const NAMESPACE: &'static str = "ta::third_party::tradingview:::bollinger_bands_width";
// }

impl Incremental<(), BollingerBandsWidthFeatures> for BollingerBandsWidthFeatureBuilder {
    fn next(&mut self, _: ()) -> BollingerBandsWidthFeatures {
        let value = self.inner.next(());

        self.features.value = value;
        return self.features.clone();
    }
}

impl Incremental<(), Box<dyn Features>> for BollingerBandsWidthFeatureBuilder {
    fn next(&mut self, _: ()) -> Box<dyn Features> {
        return Box::new(Incremental::<(), BollingerBandsWidthFeatures>::next(
            self,
            (),
        ));
    }
}
