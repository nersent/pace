use crate::components::{
    component::Component, component_context::ComponentContext,
    window_cache_component::WindowCacheComponent,
};

use super::bars::lowest;

/// Lowest value for a given number of bars back.
///
/// Same as PineScript `ta.lowest(src)`. Similar to `ta.lowest(src, length)`, but `length` is fixed and set on initialization.
pub struct LowestComponent {
    pub length: usize,
    pub ctx: ComponentContext,
    input_cache: WindowCacheComponent<Option<f64>>,
}

impl LowestComponent {
    pub fn new(ctx: ComponentContext, length: usize) -> Self {
        assert!(
            length >= 1,
            "LowestComponent must have a length of at least 1"
        );
        return Self {
            ctx: ctx.clone(),
            length,
            input_cache: WindowCacheComponent::new(ctx.clone(), length),
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
