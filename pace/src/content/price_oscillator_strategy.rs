use crate::{
    components::{component::Component, component_context::ComponentContext},
    strategy::trade::TradeDirection,
    ta::{
        cross_over_threshold_component::CrossOverThresholdComponent,
        cross_under_threshold_component::CrossUnderThresholdComponent,
    },
};

pub static PO_THRESHOLD_OVERSOLD: f64 = 0.0;
pub static PO_THRESHOLD_OVERBOUGHT: f64 = 0.0;

pub struct PoStrategyConfig {
    pub threshold_oversold: f64,
    pub threshold_overbought: f64,
}

impl Default for PoStrategyConfig {
    fn default() -> Self {
        return Self {
            threshold_oversold: PO_THRESHOLD_OVERSOLD,
            threshold_overbought: PO_THRESHOLD_OVERBOUGHT,
        };
    }
}

/// Custom Price Oscillator Strategy. May be incorrect.
pub struct PoStrategy {
    pub config: PoStrategyConfig,
    pub ctx: ComponentContext,
    cross_over: CrossOverThresholdComponent,
    cross_under: CrossUnderThresholdComponent,
}

impl PoStrategy {
    pub fn new(ctx: ComponentContext, config: PoStrategyConfig) -> Self {
        return PoStrategy {
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

impl Component<Option<f64>, Option<TradeDirection>> for PoStrategy {
    fn next(&mut self, po: Option<f64>) -> Option<TradeDirection> {
        let is_cross_over = self.cross_over.next(po);
        let is_cross_under = self.cross_under.next(po);

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
