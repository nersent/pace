use crate::components::{component_context::ComponentContext, source::Source};

use super::rsi_component::{RelativeStrengthIndexComponent, RelativeStrengthIndexComponentResult};

pub struct RelativeStrengthIndexIndicatorConfig {
    pub length: usize,
    pub src: Source,
}

pub struct RelativeStrengthIndexIndicator {
    config: RelativeStrengthIndexIndicatorConfig,
    ctx: ComponentContext,
    rsi: RelativeStrengthIndexComponent,
}

pub type RelativeStrengthIndexIndicatorResult = RelativeStrengthIndexComponentResult;

impl RelativeStrengthIndexIndicator {
    pub fn new(ctx: ComponentContext, config: RelativeStrengthIndexIndicatorConfig) -> Self {
        return RelativeStrengthIndexIndicator {
            ctx: ctx.clone(),
            rsi: RelativeStrengthIndexComponent::new(ctx.clone(), config.length),
            config,
        };
    }

    pub fn next(&mut self) -> RelativeStrengthIndexIndicatorResult {
        self.ctx.assert();
        let src = self.config.src.get();
        return self.rsi.next(src);
    }
}
