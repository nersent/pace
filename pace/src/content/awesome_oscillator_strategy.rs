use crate::{
    components::{component::Component, component_context::ComponentContext},
    strategy::trade::TradeDirection,
    ta::{
        cross_over_threshold_component::CrossOverThresholdComponent,
        cross_under_threshold_component::CrossUnderThresholdComponent,
    },
};

pub static AO_THRESHOLD_OVERSOLD: f64 = 0.0;
pub static AO_THRESHOLD_OVERBOUGHT: f64 = 0.0;

pub struct AoStrategyConfig {
    pub threshold_oversold: f64,
    pub threshold_overbought: f64,
}

impl Default for AoStrategyConfig {
    fn default() -> Self {
        return Self {
            threshold_oversold: AO_THRESHOLD_OVERSOLD,
            threshold_overbought: AO_THRESHOLD_OVERBOUGHT,
        };
    }
}

pub struct AoStrategy {
    pub config: AoStrategyConfig,
    pub ctx: ComponentContext,
    cross_over: CrossOverThresholdComponent,
    cross_under: CrossUnderThresholdComponent,
}

impl AoStrategy {
    pub fn new(ctx: ComponentContext, config: AoStrategyConfig) -> Self {
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

impl Component<Option<f64>, Option<TradeDirection>> for AoStrategy {
    fn next(&mut self, ao: Option<f64>) -> Option<TradeDirection> {
        let is_cross_over = self.cross_over.next(ao);
        let is_cross_under = self.cross_under.next(ao);

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
