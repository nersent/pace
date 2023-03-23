use crate::{
    common::src::{AnySrc, Src, SrcKind},
    core::{
        context::Context,
        incremental::{Incremental, IncrementalDefault},
    },
    pinescript::common::ps_div,
    strategy::trade::TradeDirection,
    ta::{
        cross::Cross,
        cross_over_threshold::CrossOverThreshold,
        cross_under_threshold::CrossUnderThreshold,
        highest_bars::HighestBars,
        lowest_bars::LowestBars,
        moving_average::{AnyMa, Ma, MaKind},
        sum::Sum,
    },
};

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

impl Incremental<(), Option<f64>> for ChaikinMoneyFlow {
    fn next(&mut self, _: ()) -> Option<f64> {
        let close = self.ctx.bar.close();
        let high = self.ctx.bar.high();
        let low = self.ctx.bar.low();
        let volume = self.ctx.bar.volume();

        let volume_sum = self.volume_sum.next(volume);

        let ad: Option<f64> = match (close, high, low, volume) {
            (Some(close), Some(high), Some(low), Some(volume)) => {
                if close == high && close == low || high == low {
                    Some(0.0)
                } else {
                    Some(((2.0 * close - low - high) / (high - low)) * volume)
                }
            }
            _ => None,
        };

        let ad_sum = self.ad_sum.next(ad);

        let cmf = ps_div(ad_sum, volume_sum);

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

impl Incremental<Option<f64>, Option<TradeDirection>> for ChaikinMoneyFlowStrategy {
    fn next(&mut self, cmf: Option<f64>) -> Option<TradeDirection> {
        let is_cross_over = self.cross_over.next(cmf);
        let is_cross_under = self.cross_under.next(cmf);

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
