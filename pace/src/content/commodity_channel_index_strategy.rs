use crate::{
    components::{component::Component, component_context::ComponentContext},
    strategy::trade::TradeDirection,
    ta::{
        cross_over_threshold_component::CrossOverThresholdComponent,
        cross_under_threshold_component::CrossUnderThresholdComponent,
    },
};

pub static CCI_THRESHOLD_OVERSOLD: f64 = -200.0;
pub static CCI_THRESHOLD_OVERBOUGHT: f64 = 200.0;

pub struct CciStrategyConfig {
    pub threshold_oversold: f64,
    pub threshold_overbought: f64,
}

impl Default for CciStrategyConfig {
    fn default() -> Self {
        return Self {
            threshold_oversold: CCI_THRESHOLD_OVERSOLD,
            threshold_overbought: CCI_THRESHOLD_OVERBOUGHT,
        };
    }
}

/// Custom Commodity Channel Index Strategy. May be incorrect.
pub struct CciStrategy {
    pub config: CciStrategyConfig,
    pub ctx: ComponentContext,
    cross_over: CrossOverThresholdComponent,
    cross_under: CrossUnderThresholdComponent,
}

impl CciStrategy {
    pub fn new(ctx: ComponentContext, config: CciStrategyConfig) -> Self {
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

impl Component<Option<f64>, Option<TradeDirection>> for CciStrategy {
    fn next(&mut self, cci: Option<f64>) -> Option<TradeDirection> {
        let is_cross_over = self.cross_over.next(cci);
        let is_cross_under = self.cross_under.next(cci);

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
