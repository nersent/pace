use crate::{
    components::{component::Component, component_context::ComponentContext},
    strategy::trade::TradeDirection,
    ta::{
        cross_over_threshold_component::CrossOverThresholdComponent,
        cross_under_threshold_component::CrossUnderThresholdComponent,
    },
};

pub static VO_THRESHOLD_OVERSOLD: f64 = 0.0;
pub static VO_THRESHOLD_OVERBOUGHT: f64 = 0.0;

pub struct VoStrategyConfig {
    pub threshold_oversold: f64,
    pub threshold_overbought: f64,
}

impl Default for VoStrategyConfig {
    fn default() -> Self {
        return Self {
            threshold_oversold: VO_THRESHOLD_OVERSOLD,
            threshold_overbought: VO_THRESHOLD_OVERBOUGHT,
        };
    }
}

pub struct VoStrategy {
    pub ctx: ComponentContext,
    cross_over: CrossOverThresholdComponent,
    cross_under: CrossUnderThresholdComponent,
}

impl VoStrategy {
    pub fn new(ctx: ComponentContext, config: VoStrategyConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            cross_over: CrossOverThresholdComponent::new(ctx.clone(), config.threshold_oversold),
            cross_under: CrossUnderThresholdComponent::new(
                ctx.clone(),
                config.threshold_overbought,
            ),
        };
    }
}

impl Component<Option<f64>, Option<TradeDirection>> for VoStrategy {
    fn next(&mut self, vo: Option<f64>) -> Option<TradeDirection> {
        let is_cross_over = self.cross_over.next(vo);
        let is_cross_under = self.cross_under.next(vo);

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
