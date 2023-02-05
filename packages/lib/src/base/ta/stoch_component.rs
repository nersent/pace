use crate::base::components::{
    common::fixed_value_cache_component::FixedValueCacheComponent,
    component_context::ComponentContext,
};

use super::stoch::compute_stoch;

pub struct StochComponent {
    length: usize,
    ctx: ComponentContext,
    prev_stoch: Option<f64>,
    high_input_cache: FixedValueCacheComponent,
    low_input_cache: FixedValueCacheComponent,
}

impl StochComponent {
    pub fn new(ctx: ComponentContext, length: usize) -> Self {
        assert!(length >= 1, "StochComponent length must be >= 1");
        return StochComponent {
            ctx: ctx.clone(),
            length,
            prev_stoch: None,
            high_input_cache: FixedValueCacheComponent::new(ctx.clone(), length),
            low_input_cache: FixedValueCacheComponent::new(ctx.clone(), length),
        };
    }

    pub fn next(&mut self, value: Option<f64>, high: Option<f64>, low: Option<f64>) -> Option<f64> {
        self.ctx.on_next();

        self.high_input_cache.next(high);
        self.low_input_cache.next(low);

        if !self.ctx.at_length(self.length) {
            return None;
        }

        let stoch = compute_stoch(
            value,
            self.high_input_cache.all(),
            self.low_input_cache.all(),
            self.prev_stoch,
        );
        self.prev_stoch = stoch;

        return stoch;
    }
}
