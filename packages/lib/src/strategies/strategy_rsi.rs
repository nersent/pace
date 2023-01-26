use crate::{
    base::{
        component_context::ComponentContext,
        implicit::{
            recursive::{
                recursive_cross_over::RecursiveCrossOver,
                recursive_cross_under::RecursiveCrossUnder,
                recursive_rsi::{RecursiveRSI, RecursiveRSIResult},
            },
            source::Source,
        },
        strategy::types::StrategyActionKind,
    },
    indicators::indicator_rsi::{IndicatorRSI, IndicatorRSIResult},
};

pub struct StrategyRSIConfig {
    pub oversold_threshold: f64,
    pub overbought_threshold: f64,
}

pub struct StrategyRSI {
    pub config: StrategyRSIConfig,
    ctx: ComponentContext,
    rsi: IndicatorRSI,
    cross_over: RecursiveCrossOver,
    cross_under: RecursiveCrossUnder,
}

pub static STRATEGY_RSI_DEFAULT_OVERSOLD_THRESHOLD: f64 = 30.0;
pub static STRATEGY_RSI_DEFAULT_OVERBOUGHT_THRESHOLD: f64 = 70.0;

impl StrategyRSI {
    pub fn new(ctx: ComponentContext, config: StrategyRSIConfig, rsi: IndicatorRSI) -> Self {
        return StrategyRSI {
            ctx: ctx.clone(),
            rsi,
            config,
            cross_over: RecursiveCrossOver::new(ctx.clone()),
            cross_under: RecursiveCrossUnder::new(ctx.clone()),
        };
    }

    pub fn next(&mut self) -> (StrategyActionKind, IndicatorRSIResult) {
        self.ctx.assert();

        let result_rsi = self.rsi.next();

        let is_cross_over = self
            .cross_over
            .next(result_rsi.rsi, Some(self.config.oversold_threshold));

        let is_cross_under = self
            .cross_under
            .next(result_rsi.rsi, Some(self.config.overbought_threshold));

        let result = if is_cross_over {
            StrategyActionKind::Long
        } else if is_cross_under {
            StrategyActionKind::Short
        } else {
            StrategyActionKind::None
        };

        return (result, result_rsi);
    }
}
