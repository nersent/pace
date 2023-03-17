use crate::components::{
    component::Component, component_context::ComponentContext,
    window_cache_component::WindowCacheComponent,
};

/// Compares the current `source` value to its value `length` bars ago and returns the difference.
///
/// Same as PineScript `ta.change(src)`. Similar to `ta.change(src, length)`, but `length` is fixed and set on initialization.
pub struct ChangeComponent {
    pub length: usize,
    pub ctx: ComponentContext,
    input_cache: WindowCacheComponent<Option<f64>>,
}

impl ChangeComponent {
    pub fn new(ctx: ComponentContext, length: usize) -> Self {
        assert!(
            length >= 1,
            "ChangeComponent must have a length of at least 1"
        );
        return Self {
            ctx: ctx.clone(),
            length,
            input_cache: WindowCacheComponent::new(ctx.clone(), length + 1),
        };
    }
}

impl Component<Option<f64>, Option<f64>> for ChangeComponent {
    fn next(&mut self, value: Option<f64>) -> Option<f64> {
        self.input_cache.next(value);
        let first_value = self.input_cache.first_unwrapped();
        let last_value = self.input_cache.last_unwrapped();
        let is_filled = self.input_cache.is_filled();

        if !is_filled || first_value.is_none() || last_value.is_none() {
            return None;
        }
        let first_value = first_value.unwrap();
        if first_value == 0.0 {
            return None;
        }
        return Some(last_value.unwrap() - first_value);
    }
}
