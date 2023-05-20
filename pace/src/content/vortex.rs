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
        average_true_range::Atr,
        cross::{Cross, CrossMode},
        cross_over_threshold::CrossOverThreshold,
        cross_under_threshold::CrossUnderThreshold,
        highest_bars::HighestBars,
        lowest_bars::LowestBars,
        moving_average::{Ma, MaKind},
        sum::Sum,
    },
};

pub struct VortexConfig {
    pub length: usize,
}

impl Default for VortexConfig {
    fn default() -> Self {
        Self { length: 14 }
    }
}

pub struct VortexData {
    pub plus: f64,
    pub minus: f64,
}

/// Ported from https://www.tradingview.com/chart/?solution=43000591352
pub struct Vortex {
    pub config: VortexConfig,
    pub ctx: Context,
    vmp_sum: Sum,
    vmm_sum: Sum,
    atr_sum: Sum,
    atr: Atr,
}

impl Vortex {
    pub fn new(ctx: Context, config: VortexConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            vmp_sum: Sum::new(ctx.clone(), config.length),
            vmm_sum: Sum::new(ctx.clone(), config.length),
            atr_sum: Sum::new(ctx.clone(), config.length),
            atr: Atr::new(ctx.clone(), 1),
            config,
        };
    }
}

impl Incremental<(), VortexData> for Vortex {
    fn next(&mut self, _: ()) -> VortexData {
        let current_tick = self.ctx.bar.index();
        let high = self.ctx.bar.high();
        let low = self.ctx.bar.low();
        let prev_high = self.ctx.high(1);
        let prev_low = self.ctx.low(1);

        let high_prev_low_diff = f64::abs(high - prev_low);
        let low_prev_high_diff = f64::abs(low - prev_high);

        let vmp = self.vmp_sum.next(high_prev_low_diff);
        let vmm = self.vmm_sum.next(low_prev_high_diff);

        let atr = self.atr.next(());
        let str = self.atr_sum.next(atr);

        let vip = vmp / str;
        let vim = vmm / str;

        return VortexData {
            plus: vip,
            minus: vim,
        };
    }
}

pub struct VortexStrategy {
    pub ctx: Context,
    cross: Cross,
}

/// Custom Vortex Strategy. May be incorrect.
impl VortexStrategy {
    pub fn new(ctx: Context) -> Self {
        return Self {
            ctx: ctx.clone(),
            cross: Cross::new(ctx.clone()),
        };
    }
}

impl Incremental<&VortexData, StrategySignal> for VortexStrategy {
    fn next(&mut self, vi: &VortexData) -> StrategySignal {
        let vip_vim_cross = self.cross.next((vi.plus, vi.minus));

        if let Some(plus_minus_cross) = vip_vim_cross {
            if plus_minus_cross == CrossMode::Over {
                return StrategySignal::Long;
            } else if plus_minus_cross == CrossMode::Under {
                return StrategySignal::Short;
            }
        }

        return StrategySignal::Hold;
    }
}

#[derive(Debug, Clone)]
pub struct VortexFeatures {
    pub vortex_plus: f64,
    pub vortex_minus: f64,
    pub trend: Option<Trend>,
    pub signal: StrategySignal,
}

impl Default for VortexFeatures {
    fn default() -> Self {
        return Self {
            vortex_plus: f64::NAN,
            vortex_minus: f64::NAN,
            trend: None,
            signal: StrategySignal::Hold,
        };
    }
}

impl Features for VortexFeatures {
    fn flatten(&self) -> HashMap<String, FeatureValue> {
        let mut map: HashMap<String, FeatureValue> = HashMap::new();

        map.insert("vortex_plus".to_string(), self.vortex_plus.into());
        map.insert("vortex_minus".to_string(), self.vortex_minus.into());
        map.insert(
            "trend".to_string(),
            self.trend.map(|x| x.into()).unwrap_or(FeatureValue::Empty),
        );
        map.insert("signal".to_string(), self.signal.into());

        return map;
    }
}

pub struct VortexFeatureBuilder {
    pub ctx: Context,
    pub inner: Vortex,
    pub inner_strategy: VortexStrategy,
    features: VortexFeatures,
}

impl VortexFeatureBuilder {
    pub fn new(ctx: Context, inner: Vortex, inner_strategy: VortexStrategy) -> Self {
        return Self {
            inner,
            inner_strategy,
            ctx,
            features: VortexFeatures::default(),
        };
    }
}

impl IncrementalFeatureBuilder<VortexFeatures> for VortexFeatureBuilder {
    const NAMESPACE: &'static str = "ta::third_party::tradingview:::vortex";
}

impl Incremental<(), VortexFeatures> for VortexFeatureBuilder {
    fn next(&mut self, _: ()) -> VortexFeatures {
        let value = self.inner.next(());
        let signal = self.inner_strategy.next(&value);

        self.features.vortex_plus = value.plus;
        self.features.vortex_minus = value.minus;
        self.features.signal = signal;

        if signal == StrategySignal::Long {
            self.features.trend = Some(Trend::Bullish);
        } else if signal == StrategySignal::Short {
            self.features.trend = Some(Trend::Bearish);
        }

        return self.features.clone();
    }
}

impl Incremental<(), Box<dyn Features>> for VortexFeatureBuilder {
    fn next(&mut self, _: ()) -> Box<dyn Features> {
        return Box::new(Incremental::<(), VortexFeatures>::next(self, ()));
    }
}
