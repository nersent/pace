use crate::{
    components::{
        component::Component, component_context::ComponentContext,
        component_default::ComponentDefault,
    },
    strategy::trade::TradeDirection,
    ta::{
        cross_over_threshold_component::CrossOverThresholdComponent,
        cross_under_threshold_component::CrossUnderThresholdComponent,
    },
};

pub static CMO_THRESHOLD_OVERSOLD: f64 = -50.0;
pub static CMO_THRESHOLD_OVERBOUGHT: f64 = 50.0;

pub struct CmoStrategyConfig {
    pub threshold_oversold: f64,
    pub threshold_overbought: f64,
}

impl Default for CmoStrategyConfig {
    fn default() -> Self {
        return Self {
            threshold_oversold: CMO_THRESHOLD_OVERSOLD,
            threshold_overbought: CMO_THRESHOLD_OVERBOUGHT,
        };
    }
}

pub struct CmoStrategy {
    pub config: CmoStrategyConfig,
    pub ctx: ComponentContext,
    cross_over: CrossOverThresholdComponent,
    cross_under: CrossUnderThresholdComponent,
}

impl CmoStrategy {
    pub fn new(ctx: ComponentContext, config: CmoStrategyConfig) -> Self {
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

impl Component<Option<f64>, Option<TradeDirection>> for CmoStrategy {
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
