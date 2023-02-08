use pyo3::types::PyDict;

use crate::base::{
    components::{
        component_context::ComponentContext, component_default::ComponentDefault,
        python::ComponentFromPyDict,
    },
    strategy::trade::TradeDirection,
    ta::{
        cross::CrossMode, cross_component::CrossComponent,
        cross_over_threshold_component::CrossOverThresholdComponent,
        cross_threshold_component::CrossThresholdComponent,
        cross_under_threshold_component::CrossUnderThresholdComponent,
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
    cross_overbought: CrossOverThresholdComponent,
    cross_oversold: CrossUnderThresholdComponent,
}

pub static RSI_STRATEGY_THRESHOLD_OVERSOLD: f64 = 30.0;
pub static RSI_STRATEGY_THRESHOLD_OVERBOUGHT: f64 = 70.0;

impl RelativeStrengthIndexStrategy {
    pub fn new(ctx: ComponentContext, config: RelativeStrengthIndexStrategyConfig) -> Self {
        return RelativeStrengthIndexStrategy {
            ctx: ctx.clone(),
            cross_overbought: CrossOverThresholdComponent::new(
                ctx.clone(),
                config.threshold_oversold,
            ),
            cross_oversold: CrossUnderThresholdComponent::new(
                ctx.clone(),
                config.threshold_overbought,
            ),
            config,
        };
    }

    pub fn next(&mut self, rsi: Option<f64>) -> Option<TradeDirection> {
        self.ctx.assert();

        let is_cross_over = self.cross_overbought.next(rsi);
        let is_cross_under = self.cross_oversold.next(rsi);

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
