use crate::{
    components::{component::Component, component_context::ComponentContext},
    strategy::trade::TradeDirection,
    ta::{
        cross_over_threshold_component::CrossOverThresholdComponent,
        cross_under_threshold_component::CrossUnderThresholdComponent,
    },
};

pub static CRSI_THRESHOLD_OVERSOLD: f64 = 20.0;
pub static CRSI_THRESHOLD_OVERBOUGHT: f64 = 80.0;

pub struct CrsiStrategyConfig {
    pub threshold_oversold: f64,
    pub threshold_overbought: f64,
}

impl Default for CrsiStrategyConfig {
    fn default() -> Self {
        return Self {
            threshold_oversold: CRSI_THRESHOLD_OVERSOLD,
            threshold_overbought: CRSI_THRESHOLD_OVERBOUGHT,
        };
    }
}

pub struct CrsiStrategy {
    pub config: CrsiStrategyConfig,
    pub ctx: ComponentContext,
    cross_overbought: CrossOverThresholdComponent,
    cross_oversold: CrossUnderThresholdComponent,
}

impl CrsiStrategy {
    pub fn new(ctx: ComponentContext, config: CrsiStrategyConfig) -> Self {
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

impl Component<Option<f64>, Option<TradeDirection>> for CrsiStrategy {
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
