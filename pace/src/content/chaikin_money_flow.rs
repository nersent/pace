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
    strategy::trade::{StrategySignal, TradeDirection},
    ta::{
        cross::Cross, cross_over_threshold::CrossOverThreshold,
        cross_under_threshold::CrossUnderThreshold, highest_bars::HighestBars,
        lowest_bars::LowestBars, sum::Sum,
    },
    utils::float::Float64Utils,
};

pub static CHAIKIN_MONEY_FLOW_MIN_VALUE: f64 = -1.0;
pub static CHAIKIN_MONEY_FLOW_MAX_VALUE: f64 = 1.0;

pub struct ChaikinMoneyFlowConfig {
    pub length: usize,
}

impl Default for ChaikinMoneyFlowConfig {
    fn default() -> Self {
        Self { length: 20 }
    }
}

pub struct ChaikinMoneyFlow {
    pub config: ChaikinMoneyFlowConfig,
    pub ctx: Context,
    volume_sum: Sum,
    ad_sum: Sum,
}

/// Ported from https://www.tradingview.com/chart/?solution=43000501974
impl ChaikinMoneyFlow {
    pub fn new(ctx: Context, config: ChaikinMoneyFlowConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            volume_sum: Sum::new(ctx.clone(), config.length),
            ad_sum: Sum::new(ctx.clone(), config.length),
            config,
        };
    }
}

impl Incremental<(), f64> for ChaikinMoneyFlow {
    fn next(&mut self, _: ()) -> f64 {
        let close = self.ctx.bar.close();
        let high = self.ctx.bar.high();
        let low = self.ctx.bar.low();
        let volume = self.ctx.bar.volume();

        let volume_sum = self.volume_sum.next(volume);

        let ad = (((2.0 * close - low - high) / (high - low)) * volume)
            .normalize()
            .ps_nz();

        let ad_sum = self.ad_sum.next(ad);

        let cmf = ad_sum / volume_sum;

        return cmf;
    }
}

pub static CHAIKIN_MONEY_FLOW_THRESHOLD_OVERSOLD: f64 = 0.0;
pub static CHAIKIN_MONEY_FLOW_THRESHOLD_OVERBOUGHT: f64 = 0.0;

pub struct ChaikinMoneyFlowStrategyConfig {
    pub threshold_oversold: f64,
    pub threshold_overbought: f64,
}

impl Default for ChaikinMoneyFlowStrategyConfig {
    fn default() -> Self {
        return Self {
            threshold_oversold: CHAIKIN_MONEY_FLOW_THRESHOLD_OVERSOLD,
            threshold_overbought: CHAIKIN_MONEY_FLOW_THRESHOLD_OVERBOUGHT,
        };
    }
}

/// Custom Chaikin Money Flow Strategy. May be incorrect.
pub struct ChaikinMoneyFlowStrategy {
    pub config: ChaikinMoneyFlowStrategyConfig,
    pub ctx: Context,
    cross_over: CrossOverThreshold,
    cross_under: CrossUnderThreshold,
}

impl ChaikinMoneyFlowStrategy {
    pub fn new(ctx: Context, config: ChaikinMoneyFlowStrategyConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            cross_over: CrossOverThreshold::new(ctx.clone(), config.threshold_oversold),
            cross_under: CrossUnderThreshold::new(ctx.clone(), config.threshold_overbought),
            config,
        };
    }
}

impl Incremental<f64, StrategySignal> for ChaikinMoneyFlowStrategy {
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
