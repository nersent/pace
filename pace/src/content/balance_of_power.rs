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
        lowest_bars::LowestBars,
    },
    utils::float::Float64Utils,
};

pub static BALANCE_OF_POWER_MIN_VALUE: f64 = -1.0;
pub static BALANCE_OF_POWER_MAX_VALUE: f64 = 1.0;

pub struct BalanceOfPower {
    pub ctx: Context,
}

/// Balance of Power Indicator.
///
/// Ported from https://www.tradingview.com/chart/?solution=43000589100
impl BalanceOfPower {
    pub fn new(ctx: Context) -> Self {
        return Self { ctx };
    }
}

impl Incremental<(), f64> for BalanceOfPower {
    fn next(&mut self, _: ()) -> f64 {
        let close = self.ctx.bar.close();
        let open = self.ctx.bar.open();
        let high = self.ctx.bar.high();
        let low = self.ctx.bar.low();

        let value = (close - open) / (high - low).normalize();

        return value;
    }
}

pub static BALANCE_OF_POWER_THRESHOLD_OVERSOLD: f64 = 0.0;
pub static BALANCE_OF_POWER_THRESHOLD_OVERBOUGHT: f64 = 0.0;

pub struct BalanceOfPowerStrategyConfig {
    pub threshold_oversold: f64,
    pub threshold_overbought: f64,
}

impl Default for BalanceOfPowerStrategyConfig {
    fn default() -> Self {
        return Self {
            threshold_oversold: BALANCE_OF_POWER_THRESHOLD_OVERSOLD,
            threshold_overbought: BALANCE_OF_POWER_THRESHOLD_OVERBOUGHT,
        };
    }
}

/// Custom Balance of Power Strategy. May be incorrect.
pub struct BalanceOfPowerStrategy {
    pub config: BalanceOfPowerStrategyConfig,
    pub ctx: Context,
    cross_over: CrossOverThreshold,
    cross_under: CrossUnderThreshold,
}

impl BalanceOfPowerStrategy {
    pub fn new(ctx: Context, config: BalanceOfPowerStrategyConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            cross_over: CrossOverThreshold::new(ctx.clone(), config.threshold_oversold),
            cross_under: CrossUnderThreshold::new(ctx.clone(), config.threshold_overbought),
            config,
        };
    }
}

impl Incremental<f64, StrategySignal> for BalanceOfPowerStrategy {
    fn next(&mut self, value: f64) -> StrategySignal {
        let is_cross_over = self.cross_over.next(value);
        let is_cross_under = self.cross_under.next(value);

        if is_cross_over {
            return StrategySignal::Long;
        }
        if is_cross_under {
            return StrategySignal::Short;
        }
        return StrategySignal::Hold;
    }
}
