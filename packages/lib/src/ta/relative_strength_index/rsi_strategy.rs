use crate::{
    components::component_context::ComponentContext,
    strategy::action::StrategyActionKind,
    ta::cross::{cross::CrossMode, cross_component::CrossComponent},
};

use super::rsi_indicator::{RelativeStrengthIndexIndicator, RelativeStrengthIndexIndicatorResult};

pub struct RelativeStrengthIndexStrategyConfig {
    pub threshold_oversold: f64,
    pub threshold_overbought: f64,
}

pub struct RelativeStrengthIndexStrategy {
    pub config: RelativeStrengthIndexStrategyConfig,
    ctx: ComponentContext,
    rsi: RelativeStrengthIndexIndicator,
    cross_over: CrossComponent,
    cross_under: CrossComponent,
}

pub static RSI_STRATEGY_THRESHOLD_OVERSOLD: f64 = 30.0;
pub static RSI_STRATEGY_THRESHOLD_OVERBOUGHT: f64 = 70.0;

impl RelativeStrengthIndexStrategy {
    pub fn new(
        ctx: ComponentContext,
        config: RelativeStrengthIndexStrategyConfig,
        rsi: RelativeStrengthIndexIndicator,
    ) -> Self {
        return RelativeStrengthIndexStrategy {
            ctx: ctx.clone(),
            rsi,
            config,
            cross_over: CrossComponent::new(ctx.clone(), CrossMode::Over),
            cross_under: CrossComponent::new(ctx.clone(), CrossMode::Under),
        };
    }

    pub fn next(&mut self) -> (StrategyActionKind, RelativeStrengthIndexIndicatorResult) {
        self.ctx.assert();

        let result_rsi = self.rsi.next();

        let is_cross_over = self
            .cross_over
            .next(result_rsi.rsi, Some(self.config.threshold_oversold));

        let is_cross_under = self
            .cross_under
            .next(result_rsi.rsi, Some(self.config.threshold_overbought));

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
