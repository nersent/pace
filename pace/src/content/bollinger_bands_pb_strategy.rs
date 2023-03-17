use crate::{
    components::{component::Component, component_context::ComponentContext},
    strategy::trade::TradeDirection,
    ta::{
        cross_over_threshold_component::CrossOverThresholdComponent,
        cross_under_threshold_component::CrossUnderThresholdComponent,
    },
};

pub static BBPB_THRESHOLD_OVERSOLD: f64 = 0.0;
pub static BBPB_THRESHOLD_OVERBOUGHT: f64 = 1.0;

pub struct BbpbStrategyConfig {
    pub threshold_oversold: f64,
    pub threshold_overbought: f64,
}

impl Default for BbpbStrategyConfig {
    fn default() -> Self {
        return Self {
            threshold_overbought: BBPB_THRESHOLD_OVERBOUGHT,
            threshold_oversold: BBPB_THRESHOLD_OVERSOLD,
        };
    }
}

/// Bollinger Bands %B Strategy.
///
/// Ported from https://www.tradingview.com/chart/?solution=43000589104
pub struct BbpbStrategy {
    pub config: BbpbStrategyConfig,
    pub ctx: ComponentContext,
    cross_over: CrossOverThresholdComponent,
    cross_under: CrossUnderThresholdComponent,
}

impl BbpbStrategy {
    pub fn new(ctx: ComponentContext, config: BbpbStrategyConfig) -> Self {
        return BbpbStrategy {
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

impl Component<Option<f64>, Option<TradeDirection>> for BbpbStrategy {
    fn next(&mut self, bbpb: Option<f64>) -> Option<TradeDirection> {
        let is_cross_over = self.cross_over.next(bbpb);
        let is_cross_under = self.cross_under.next(bbpb);

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
