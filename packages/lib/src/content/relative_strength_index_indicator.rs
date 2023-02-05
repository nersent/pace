use crate::base::{
    asset::source::{Source, SourceKind},
    components::{component_context::ComponentContext, component_default::ComponentDefault},
    ta::rsi_component::{RelativeStrengthIndexComponent, RelativeStrengthIndexComponentMetadata},
};

pub struct RelativeStrengthIndexIndicatorConfig {
    pub length: usize,
    pub src: Source,
}

impl ComponentDefault for RelativeStrengthIndexIndicatorConfig {
    fn default(ctx: ComponentContext) -> Self {
        return RelativeStrengthIndexIndicatorConfig {
            length: 14,
            src: Source::from_kind(ctx, SourceKind::Close),
        };
    }
}

pub struct RelativeStrengthIndexIndicator {
    rsi: RelativeStrengthIndexComponent,
    config: RelativeStrengthIndexIndicatorConfig,
    ctx: ComponentContext,
}

impl RelativeStrengthIndexIndicator {
    pub fn new(ctx: ComponentContext, config: RelativeStrengthIndexIndicatorConfig) -> Self {
        return RelativeStrengthIndexIndicator {
            ctx: ctx.clone(),
            rsi: RelativeStrengthIndexComponent::new(ctx.clone(), config.length),
            config,
        };
    }

    pub fn metadata(&self) -> &RelativeStrengthIndexComponentMetadata {
        return &self.rsi.metadata;
    }

    pub fn next(&mut self) -> Option<f64> {
        self.ctx.assert();
        let src = self.config.src.get();
        return self.rsi.next(src);
    }
}
