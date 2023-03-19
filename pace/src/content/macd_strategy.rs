use crate::{
    components::{component::Component, component_context::ComponentContext},
    strategy::trade::TradeDirection,
    ta::{
        cross_over_threshold_component::CrossOverThresholdComponent,
        cross_under_threshold_component::CrossUnderThresholdComponent,
    },
};

pub struct MacdStrategy {
    pub ctx: ComponentContext,
    cross_over: CrossOverThresholdComponent,
    cross_under: CrossUnderThresholdComponent,
}

impl MacdStrategy {
    pub fn new(ctx: ComponentContext) -> Self {
        return Self {
            ctx: ctx.clone(),
            cross_over: CrossOverThresholdComponent::new(ctx.clone(), 0.0),
            cross_under: CrossUnderThresholdComponent::new(ctx.clone(), 0.0),
        };
    }
}

impl Component<Option<f64>, Option<TradeDirection>> for MacdStrategy {
    fn next(&mut self, macd_delta: Option<f64>) -> Option<TradeDirection> {
        let cross_over = self.cross_over.next(macd_delta);
        let cross_under = self.cross_under.next(macd_delta);

        if cross_over {
            return Some(TradeDirection::Long);
        } else if cross_under {
            return Some(TradeDirection::Short);
        }

        return None;
    }
}
