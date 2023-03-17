use crate::{
    components::{component::Component, component_context::ComponentContext},
    strategy::trade::TradeDirection,
    ta::{
        cross_over_threshold_component::CrossOverThresholdComponent,
        cross_under_threshold_component::CrossUnderThresholdComponent,
    },
};

pub static RSI_THRESHOLD_OVERSOLD: f64 = 30.0;
pub static RSI_THRESHOLD_OVERBOUGHT: f64 = 70.0;

pub struct RsiStrategyConfig {
    pub threshold_oversold: f64,
    pub threshold_overbought: f64,
}

impl Default for RsiStrategyConfig {
    fn default() -> Self {
        return Self {
            threshold_oversold: RSI_THRESHOLD_OVERSOLD,
            threshold_overbought: RSI_THRESHOLD_OVERBOUGHT,
        };
    }
}

pub struct RsiStrategy {
    pub config: RsiStrategyConfig,
    pub ctx: ComponentContext,
    cross_overbought: CrossOverThresholdComponent,
    cross_oversold: CrossUnderThresholdComponent,
}

impl RsiStrategy {
    pub fn new(ctx: ComponentContext, config: RsiStrategyConfig) -> Self {
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

impl Component<Option<f64>, Option<TradeDirection>> for RsiStrategy {
    fn next(&mut self, rsi: Option<f64>) -> Option<TradeDirection> {
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
