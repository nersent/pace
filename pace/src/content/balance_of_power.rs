use crate::{
    common::src::{AnySrc, Src, SrcKind},
    core::{
        context::Context,
        incremental::{Incremental, IncrementalDefault},
    },
    strategy::trade::TradeDirection,
    ta::{
        cross::Cross,
        cross_over_threshold::CrossOverThreshold,
        cross_under_threshold::CrossUnderThreshold,
        highest_bars::HighestBars,
        lowest_bars::LowestBars,
        moving_average::{AnyMa, Ma, MaKind},
    },
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

impl Incremental<(), Option<f64>> for BalanceOfPower {
    fn next(&mut self, _: ()) -> Option<f64> {
        let close = self.ctx.bar.close();
        let open = self.ctx.bar.open();
        let high = self.ctx.bar.high();
        let low = self.ctx.bar.low();

        let value = match (close, open, high, low) {
            (Some(close), Some(open), Some(high), Some(low)) => {
                if high == low {
                    return None;
                }

                return Some((close - open) / (high - low));
            }
            _ => None,
        };

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

impl Incremental<Option<f64>, Option<TradeDirection>> for BalanceOfPowerStrategy {
    fn next(&mut self, value: Option<f64>) -> Option<TradeDirection> {
        let is_cross_over = self.cross_over.next(value);
        let is_cross_under = self.cross_under.next(value);

        let result = if is_cross_over {
            Some(TradeDirection::Long)
        } else if is_cross_under {
            Some(TradeDirection::Short)
        } else {
            None
        };

        return result;
    }
}
