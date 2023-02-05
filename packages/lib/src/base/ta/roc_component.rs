use crate::base::components::{
    common::fixed_value_cache_component::FixedValueCacheComponent,
    component_context::ComponentContext,
};

pub struct RateOfChangeComponent {
    length: usize,
    ctx: ComponentContext,
    input_cache: FixedValueCacheComponent,
}

impl RateOfChangeComponent {
    pub fn new(ctx: ComponentContext, length: usize) -> Self {
        assert!(length >= 1, "RecursiveRateOfChange length must be >= 1");
        return RateOfChangeComponent {
            ctx: ctx.clone(),
            length,
            input_cache: FixedValueCacheComponent::new(ctx.clone(), length + 1),
        };
    }

    pub fn next(&mut self, value: Option<f64>) -> Option<f64> {
        self.ctx.assert();
        let ctx = self.ctx.get();

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
        let last_value = last_value.unwrap();
        return Some(100.0 * (last_value - first_value) / first_value);
    }
}
