use crate::base::components::component_context::ComponentContext;

use super::strategy_component_context::StrategyComponentContext;

pub trait StrategyComponentDefault {
    fn default(ctx: ComponentContext, sctx: StrategyComponentContext) -> Self;
}
