use crate::{components::component_context::ComponentContext, strategy::trade::TradeDirection};

pub struct CiStrategyConfig {
    pub threshold_trend: f64,
    pub threshold_sideways: f64,
}

impl Default for CiStrategyConfig {
    fn default() -> Self {
        return Self {
            threshold_trend: CHOPPINESS_INDEX_STRATEGY_THRESHOLD_TREND,
            threshold_sideways: CHOPPINESS_INDEX_STRATEGY_THRESHOLD_SIDEWAYS,
        };
    }
}

pub struct CiStrategy {
    pub config: CiStrategyConfig,
    ctx: ComponentContext,
}

pub static CHOPPINESS_INDEX_STRATEGY_THRESHOLD_TREND: f64 = 38.2;
pub static CHOPPINESS_INDEX_STRATEGY_THRESHOLD_SIDEWAYS: f64 = 61.8;

impl CiStrategy {
    pub fn new(ctx: ComponentContext, config: CiStrategyConfig) -> Self {
        todo!("Not implemented");
        return Self {
            ctx: ctx.clone(),
            config,
        };
    }

    pub fn next(&mut self, cmf: Option<f64>) -> Option<TradeDirection> {
        return None;
    }
}
