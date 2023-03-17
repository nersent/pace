use crate::{
    components::{component::Component, component_context::ComponentContext},
    strategy::trade::TradeDirection,
    ta::{
        cross_over_threshold_component::CrossOverThresholdComponent,
        cross_under_threshold_component::CrossUnderThresholdComponent,
    },
};

pub static CMF_THRESHOLD_OVERSOLD: f64 = 0.0;
pub static CMF_THRESHOLD_OVERBOUGHT: f64 = 0.0;

pub struct CmfStrategyConfig {
    pub threshold_oversold: f64,
    pub threshold_overbought: f64,
}

impl Default for CmfStrategyConfig {
    fn default() -> Self {
        return CmfStrategyConfig {
            threshold_oversold: CMF_THRESHOLD_OVERSOLD,
            threshold_overbought: CMF_THRESHOLD_OVERBOUGHT,
        };
    }
}

pub struct CmfStrategy {
    pub config: CmfStrategyConfig,
    pub ctx: ComponentContext,
    cross_over: CrossOverThresholdComponent,
    cross_under: CrossUnderThresholdComponent,
}

impl CmfStrategy {
    pub fn new(ctx: ComponentContext, config: CmfStrategyConfig) -> Self {
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

impl Component<Option<f64>, Option<TradeDirection>> for CmfStrategy {
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
