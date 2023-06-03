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
        cross::Cross,
        cross_over_threshold::CrossOverThreshold,
        cross_under_threshold::CrossUnderThreshold,
        highest_bars::HighestBars,
        lowest_bars::LowestBars,
        moving_average::{Ma, MaKind},
        sum::Sum,
    },
};

pub static ULTIMATE_OSCILLATOR_MIN_VALUE: f64 = 0.0;
pub static ULTIMATE_OSCILLATOR_MAX_VALUE: f64 = 100.0;

pub struct UltimateOscillatorConfig {
    pub short_length: usize,
    pub mid_length: usize,
    pub long_length: usize,
}

impl Default for UltimateOscillatorConfig {
    fn default() -> Self {
        Self {
            short_length: 7,
            mid_length: 14,
            long_length: 28,
        }
    }
}

/// Ported from https://www.tradingview.com/chart/?solution=43000502328
pub struct UltimateOscillator {
    pub config: UltimateOscillatorConfig,
    pub ctx: Context,
    short_sum_bp: Sum,
    short_sum_tr: Sum,
    mid_sum_bp: Sum,
    mid_sum_tr: Sum,
    long_sum_bp: Sum,
    long_sum_tr: Sum,
}

impl UltimateOscillator {
    pub fn new(ctx: Context, config: UltimateOscillatorConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            short_sum_bp: Sum::new(ctx.clone(), config.short_length),
            short_sum_tr: Sum::new(ctx.clone(), config.short_length),
            mid_sum_bp: Sum::new(ctx.clone(), config.mid_length),
            mid_sum_tr: Sum::new(ctx.clone(), config.mid_length),
            long_sum_bp: Sum::new(ctx.clone(), config.long_length),
            long_sum_tr: Sum::new(ctx.clone(), config.long_length),
            config,
        };
    }
}

impl Incremental<(), f64> for UltimateOscillator {
    fn next(&mut self, _: ()) -> f64 {
        let high = self.ctx.bar.high();
        let low = self.ctx.bar.low();
        let close = self.ctx.bar.close();

        let prev_close = self.ctx.close(1);

        let high_ = f64::ps_max(high, prev_close);
        let low_ = f64::ps_min(low, prev_close);
        let bp = close - low_;
        let tr_ = high_ - low_;

        let fast_bp_sum = self.short_sum_bp.next(bp);
        let fast_tr_sum = self.short_sum_tr.next(tr_);

        let mid_bp_sum = self.mid_sum_bp.next(bp);
        let mid_tr_sum = self.mid_sum_tr.next(tr_);

        let slow_bp_sum = self.long_sum_bp.next(bp);
        let slow_tr_sum = self.long_sum_tr.next(tr_);

        let fast = fast_bp_sum / fast_tr_sum;
        let mid = mid_bp_sum / mid_tr_sum;
        let slow = slow_bp_sum / slow_tr_sum;

        let uo = 100.0 * (4.0 * fast + 2.0 * mid + slow) / 7.0;

        return uo;
    }
}

pub static ULTIMATE_OSCILLATOR_THRESHOLD_OVERSOLD: f64 = 50.0;
pub static ULTIMATE_OSCILLATOR_THRESHOLD_OVERBOUGHT: f64 = 50.0;

pub struct UltimateOscillatorStrategyConfig {
    pub threshold_oversold: f64,
    pub threshold_overbought: f64,
}

impl Default for UltimateOscillatorStrategyConfig {
    fn default() -> Self {
        return Self {
            threshold_oversold: ULTIMATE_OSCILLATOR_THRESHOLD_OVERSOLD,
            threshold_overbought: ULTIMATE_OSCILLATOR_THRESHOLD_OVERBOUGHT,
        };
    }
}

/// Custom Ultimate Oscillator Strategy. May be incorrect.
pub struct UltimateOscillatorStrategy {
    pub config: UltimateOscillatorStrategyConfig,
    pub ctx: Context,
    cross_over: CrossOverThreshold,
    cross_under: CrossUnderThreshold,
}

impl UltimateOscillatorStrategy {
    pub fn new(ctx: Context, config: UltimateOscillatorStrategyConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            cross_over: CrossOverThreshold::new(ctx.clone(), config.threshold_oversold),
            cross_under: CrossUnderThreshold::new(ctx.clone(), config.threshold_overbought),
            config,
        };
    }
}

impl Incremental<f64, StrategySignal> for UltimateOscillatorStrategy {
    fn next(&mut self, po: f64) -> StrategySignal {
        let is_cross_over = self.cross_over.next(po);
        let is_cross_under = self.cross_under.next(po);

        if is_cross_over {
            return StrategySignal::Long;
        }
        if is_cross_under {
            return StrategySignal::Short;
        }
        return StrategySignal::Hold;
    }
}
