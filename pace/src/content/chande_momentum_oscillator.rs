use std::collections::HashMap;

use crate::{
    common::src::{AnySrc, Src, SrcKind},
    core::{
        context::Context,
        features::{FeatureValue, Features, IncrementalFeatureBuilder},
        incremental::{Incremental, IncrementalDefault},
        trend::Trend,
    },
    pinescript::common::PineScriptFloat64,
    statistics::normalization::rescale,
    strategy::trade::{StrategySignal, TradeDirection},
    ta::{
        cross::Cross, cross_over_threshold::CrossOverThreshold,
        cross_under_threshold::CrossUnderThreshold, highest_bars::HighestBars,
        lowest_bars::LowestBars, sum::Sum,
    },
    utils::float::Float64Utils,
};

pub static CHANDE_MOMENTUM_OSCILLATOR_MIN_VALUE: f64 = -100.0;
pub static CHANDE_MOMENTUM_OSCILLATOR_MAX_VALUE: f64 = 100.0;

pub struct ChandeMomentumOscillatorConfig {
    pub length: usize,
    pub src: AnySrc,
}

impl IncrementalDefault for ChandeMomentumOscillatorConfig {
    fn default(ctx: Context) -> Self {
        Self {
            length: 9,
            src: Box::new(Src::new(ctx.clone(), SrcKind::Close)),
        }
    }
}

/// Ported from https://www.tradingview.com/chart/?solution=43000589109
pub struct ChandeMomentumOscillator {
    pub config: ChandeMomentumOscillatorConfig,
    pub ctx: Context,
    prev_src: f64,
    sm1: Sum,
    sm2: Sum,
}

impl ChandeMomentumOscillator {
    pub fn new(ctx: Context, config: ChandeMomentumOscillatorConfig) -> Self {
        assert!(
            config.length > 1,
            "ChandeMomentumOscillatorIndicator length must be greater than 1"
        );
        return Self {
            ctx: ctx.clone(),
            prev_src: f64::NAN,
            sm1: Sum::new(ctx.clone(), config.length),
            sm2: Sum::new(ctx.clone(), config.length),
            config,
        };
    }
}

impl Incremental<(), f64> for ChandeMomentumOscillator {
    fn next(&mut self, _: ()) -> f64 {
        let src = self.config.src.next(());
        let momm = src - self.prev_src;

        let m1 = f64::ps_max(0.0, momm);
        let m2 = f64::abs(f64::ps_min(0.0, momm));

        let sm1 = self.sm1.next(m1);
        let sm2 = self.sm2.next(m2);

        if sm1 == -sm2 {
            return f64::NAN;
        }

        let chande_mo = 100.0 * (sm1 - sm2) / (sm1 + sm2);
        self.prev_src = src;

        return chande_mo;
    }
}

pub static CHANDE_MOMENTUM_OSCILLATOR_THRESHOLD_OVERSOLD: f64 = -50.0;
pub static CHANDE_MOMENTUM_OSCILLATOR_THRESHOLD_OVERBOUGHT: f64 = 50.0;

pub struct ChandeMomentumOscillatorStrategyConfig {
    pub threshold_oversold: f64,
    pub threshold_overbought: f64,
}

impl Default for ChandeMomentumOscillatorStrategyConfig {
    fn default() -> Self {
        return Self {
            threshold_oversold: CHANDE_MOMENTUM_OSCILLATOR_THRESHOLD_OVERSOLD,
            threshold_overbought: CHANDE_MOMENTUM_OSCILLATOR_THRESHOLD_OVERBOUGHT,
        };
    }
}

/// Custom Chande Momentum Oscillator Strategy. May be incorrect.
pub struct ChandeMomentumOscillatorStrategy {
    pub config: ChandeMomentumOscillatorStrategyConfig,
    pub ctx: Context,
    cross_over: CrossOverThreshold,
    cross_under: CrossUnderThreshold,
}

impl ChandeMomentumOscillatorStrategy {
    pub fn new(ctx: Context, config: ChandeMomentumOscillatorStrategyConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            cross_over: CrossOverThreshold::new(ctx.clone(), config.threshold_oversold),
            cross_under: CrossUnderThreshold::new(ctx.clone(), config.threshold_overbought),
            config,
        };
    }
}

impl Incremental<f64, StrategySignal> for ChandeMomentumOscillatorStrategy {
    fn next(&mut self, cmf: f64) -> StrategySignal {
        let is_cross_over = self.cross_over.next(cmf);
        let is_cross_under = self.cross_under.next(cmf);

        if is_cross_over {
            return StrategySignal::Long;
        }
        if is_cross_under {
            return StrategySignal::Short;
        }
        return StrategySignal::Hold;
    }
}
