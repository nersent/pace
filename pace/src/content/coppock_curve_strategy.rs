use crate::{
    components::{component::Component, component_context::ComponentContext},
    strategy::trade::TradeDirection,
    ta::{
        cross_over_threshold_component::CrossOverThresholdComponent,
        cross_under_threshold_component::CrossUnderThresholdComponent,
    },
};

pub static CC_THRESHOLD_OVERSOLD: f64 = 0.0;
pub static CC_THRESHOLD_OVERBOUGHT: f64 = 0.0;

pub struct CcStrategyConfig {
    pub threshold_oversold: f64,
    pub threshold_overbought: f64,
}

impl Default for CcStrategyConfig {
    fn default() -> Self {
        return Self {
            threshold_oversold: CC_THRESHOLD_OVERSOLD,
            threshold_overbought: CC_THRESHOLD_OVERBOUGHT,
        };
    }
}

/// Custom Coppock Curve Strategy. May be incorrect.
pub struct CcStrategy {
    pub config: CcStrategyConfig,
    pub ctx: ComponentContext,
    cross_over: CrossOverThresholdComponent,
    cross_under: CrossUnderThresholdComponent,
}

impl CcStrategy {
    pub fn new(ctx: ComponentContext, config: CcStrategyConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            cross_over: CrossOverThresholdComponent::new(ctx.clone(), config.threshold_oversold),
            cross_under: CrossUnderThresholdComponent::new(
                ctx.clone(),
                config.threshold_overbought,
            ),
            config,
        };
    }
}

impl Component<Option<f64>, Option<TradeDirection>> for CcStrategy {
    fn next(&mut self, cc: Option<f64>) -> Option<TradeDirection> {
        let is_cross_over = self.cross_over.next(cc);
        let is_cross_under = self.cross_under.next(cc);

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
