use crate::{
    components::{component::Component, component_context::ComponentContext},
    strategy::trade::TradeDirection,
    ta::{
        cross_over_threshold_component::CrossOverThresholdComponent,
        cross_under_threshold_component::CrossUnderThresholdComponent,
    },
};

pub static WPR_THRESHOLD_OVERSOLD: f64 = -80.0;
pub static WPR_THRESHOLD_OVERBOUGHT: f64 = -20.0;

pub struct WprStrategyConfig {
    pub threshold_oversold: f64,
    pub threshold_overbought: f64,
}

impl Default for WprStrategyConfig {
    fn default() -> Self {
        return Self {
            threshold_oversold: WPR_THRESHOLD_OVERSOLD,
            threshold_overbought: WPR_THRESHOLD_OVERBOUGHT,
        };
    }
}

pub struct WprStrategy {
    pub config: WprStrategyConfig,
    pub ctx: ComponentContext,
    cross_overbought: CrossOverThresholdComponent,
    cross_oversold: CrossUnderThresholdComponent,
}

impl WprStrategy {
    pub fn new(ctx: ComponentContext, config: WprStrategyConfig) -> Self {
        return WprStrategy {
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

impl Component<Option<f64>, Option<TradeDirection>> for WprStrategy {
    fn next(&mut self, wpr: Option<f64>) -> Option<TradeDirection> {
        let is_cross_over = self.cross_overbought.next(wpr);
        let is_cross_under = self.cross_oversold.next(wpr);

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
