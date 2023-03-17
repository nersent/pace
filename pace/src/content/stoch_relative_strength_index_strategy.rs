use crate::{
    components::{component::Component, component_context::ComponentContext},
    strategy::trade::TradeDirection,
    ta::{
        cross_over_threshold_component::CrossOverThresholdComponent,
        cross_under_threshold_component::CrossUnderThresholdComponent,
    },
};

use super::stoch_relative_strength_index_indicator::SrsiIndicatorData;

pub static SRSI_THRESHOLD_OVERSOLD: f64 = 20.0;
pub static SRSI_THRESHOLD_OVERBOUGHT: f64 = 80.0;

pub struct SrsiStrategyConfig {
    pub threshold_oversold: f64,
    pub threshold_overbought: f64,
}

impl Default for SrsiStrategyConfig {
    fn default() -> Self {
        return Self {
            threshold_oversold: SRSI_THRESHOLD_OVERSOLD,
            threshold_overbought: SRSI_THRESHOLD_OVERBOUGHT,
        };
    }
}

/// Custom Stochastic Relative Strength Index Strategy. May be incorrect.
pub struct SrsiStrategy {
    pub config: SrsiStrategyConfig,
    pub ctx: ComponentContext,
    cross_overbought: CrossOverThresholdComponent,
    cross_oversold: CrossUnderThresholdComponent,
}

impl SrsiStrategy {
    pub fn new(ctx: ComponentContext, config: SrsiStrategyConfig) -> Self {
        return Self {
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
}

impl Component<&SrsiIndicatorData, Option<TradeDirection>> for SrsiStrategy {
    fn next(&mut self, stoch_rsi: &SrsiIndicatorData) -> Option<TradeDirection> {
        let is_cross_over = self.cross_overbought.next(stoch_rsi.k);
        let is_cross_under = self.cross_oversold.next(stoch_rsi.k);

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
