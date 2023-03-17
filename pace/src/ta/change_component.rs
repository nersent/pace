use crate::components::{
    component::Component, component_context::ComponentContext,
    fixed_value_cache_component::FixedValueCacheComponent,
};

/// Compares the current value to its value length bars ago and returns the difference.
pub struct ChangeComponent {
    pub length: usize,
    pub ctx: ComponentContext,
    input_cache: FixedValueCacheComponent,
}

impl ChangeComponent {
    pub fn new(ctx: ComponentContext, length: usize) -> Self {
        assert!(length >= 1, "ChangeComponent length must be >= 1");
        return Self {
            ctx: ctx.clone(),
            length,
            input_cache: FixedValueCacheComponent::new(ctx.clone(), length + 1),
        };
    }
}

impl Component<Option<f64>, Option<f64>> for ChangeComponent {
    fn next(&mut self, value: Option<f64>) -> Option<f64> {
        self.input_cache.next(value);
        let first_value = self.input_cache.first();
        let last_value = self.input_cache.last();
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
