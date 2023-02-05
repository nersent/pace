use crate::base::{
    components::{component_context::ComponentContext, component_default::ComponentDefault},
    strategy::action::StrategyActionKind,
    ta::{
        cross::CrossMode, cross_component::CrossComponent,
        rsi_component::RelativeStrengthIndexComponentMetadata,
    },
};

use super::relative_strength_index_indicator::RelativeStrengthIndexIndicator;

pub struct RelativeStrengthIndexStrategyConfig {
    pub threshold_oversold: f64,
    pub threshold_overbought: f64,
}

impl ComponentDefault for RelativeStrengthIndexStrategyConfig {
    fn default(ctx: ComponentContext) -> Self {
        return RelativeStrengthIndexStrategyConfig {
            threshold_oversold: RSI_STRATEGY_THRESHOLD_OVERSOLD,
            threshold_overbought: RSI_STRATEGY_THRESHOLD_OVERBOUGHT,
        };
    }
}

pub struct RelativeStrengthIndexStrategy {
    pub config: RelativeStrengthIndexStrategyConfig,
    ctx: ComponentContext,
    cross_over: CrossComponent,
    cross_under: CrossComponent,
}

pub static RSI_STRATEGY_THRESHOLD_OVERSOLD: f64 = 30.0;
pub static RSI_STRATEGY_THRESHOLD_OVERBOUGHT: f64 = 70.0;

impl RelativeStrengthIndexStrategy {
    pub fn new(ctx: ComponentContext, config: RelativeStrengthIndexStrategyConfig) -> Self {
        return RelativeStrengthIndexStrategy {
            ctx: ctx.clone(),
            config,
            cross_over: CrossComponent::new(ctx.clone(), CrossMode::Over),
            cross_under: CrossComponent::new(ctx.clone(), CrossMode::Under),
        };
    }

    pub fn next(&mut self, rsi: Option<f64>) -> StrategyActionKind {
        self.ctx.assert();

        let is_cross_over = self
            .cross_over
            .next(rsi, Some(self.config.threshold_oversold));

        let is_cross_under = self
            .cross_under
            .next(rsi, Some(self.config.threshold_overbought));

        let result = if is_cross_over {
            StrategyActionKind::Long
        } else if is_cross_under {
            StrategyActionKind::Short
        } else {
            StrategyActionKind::None
        };

        return result;
    }
}
