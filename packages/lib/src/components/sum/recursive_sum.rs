use crate::components::{
    batch_validator::recursive_batch_validator::RecursiveBatchValidator,
    component_context::ComponentContext, lifo::recursive_lifo::RecursiveLIFO,
    value_cache::fixed_value_cache_component::FixedValueCacheComponent,
};

pub struct RecursiveSum {
    ctx: ComponentContext,
    length: usize,
    sum: f64,
    input_cache: FixedValueCacheComponent,
    batch_validator: RecursiveBatchValidator,
}

impl RecursiveSum {
    pub fn new(ctx: ComponentContext, length: usize) -> Self {
        return RecursiveSum {
            ctx: ctx.clone(),
            length,
            sum: 0.0,
            batch_validator: RecursiveBatchValidator::new(ctx.clone(), length),
            input_cache: FixedValueCacheComponent::new(ctx.clone(), length),
        };
    }

    pub fn next(&mut self, value: Option<f64>) -> Option<f64> {
        self.ctx.on_next();

        self.input_cache.next(value);
        let first_value = self.input_cache.first();
        let last_value = self.input_cache.last();
        let is_filled = self.input_cache.is_filled();

        let is_valid = self.batch_validator.next(value);
        let mut sum: Option<f64> = None;

        if let Some(last_value) = last_value {
            self.sum += last_value;
        }
        if is_filled && is_valid {
            sum = Some(self.sum);
        }
        if let Some(first_value) = first_value {
            self.sum -= first_value;
        }
        return sum;
    }
}
