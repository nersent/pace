use crate::components::{
    component::Component, component_context::ComponentContext,
    fixed_value_cache_component::FixedValueCacheComponent,
};

use super::bars::lowest;

/// Lowest value for a given number of bars back.
pub struct LowestComponent {
    pub length: usize,
    pub ctx: ComponentContext,
    input_cache: FixedValueCacheComponent,
}

impl LowestComponent {
    pub fn new(ctx: ComponentContext, length: usize) -> Self {
        return Self {
            ctx: ctx.clone(),
            length,
            input_cache: FixedValueCacheComponent::new(ctx.clone(), length),
        };
    }
}

impl Component<Option<f64>, Option<f64>> for LowestComponent {
    fn next(&mut self, value: Option<f64>) -> Option<f64> {
        self.input_cache.next(value);

        if !self.ctx.at_length(self.length) {
            return None;
        }

        return lowest(self.input_cache.all());
    }
}
