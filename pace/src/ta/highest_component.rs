use crate::components::{
    component::Component, component_context::ComponentContext,
    fixed_value_cache_component::FixedValueCacheComponent,
};

use super::bars::highest;

/// Highest value for a given number of bars back.
pub struct HighestComponent {
    pub length: usize,
    pub ctx: ComponentContext,
    input_cache: FixedValueCacheComponent,
}

impl HighestComponent {
    pub fn new(ctx: ComponentContext, length: usize) -> Self {
        return Self {
            ctx: ctx.clone(),
            length,
            input_cache: FixedValueCacheComponent::new(ctx.clone(), length),
        };
    }
}

impl Component<Option<f64>, Option<f64>> for HighestComponent {
    fn next(&mut self, value: Option<f64>) -> Option<f64> {
        self.input_cache.next(value);

        if !self.ctx.at_length(self.length) {
            return None;
        }

        return highest(self.input_cache.all());
    }
}
