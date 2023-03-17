use crate::components::{
    batch_validator_component::BatchValidatorComponent, component::Component,
    component_context::ComponentContext, window_cache_component::WindowCacheComponent,
};

/// The sum function returns the sliding sum of last y values of x.
///
/// Same as PineScript `math.sum(src)`. Similar to `math.sum(src, length)`, but `length` is fixed and set on initialization.
pub struct SumComponent {
    pub length: usize,
    pub ctx: ComponentContext,
    sum: f64,
    input_cache: WindowCacheComponent<Option<f64>>,
    batch_validator: BatchValidatorComponent,
}

impl SumComponent {
    pub fn new(ctx: ComponentContext, length: usize) -> Self {
        assert!(length >= 1, "SumComponent must have a length of at least 1");
        return Self {
            ctx: ctx.clone(),
            length,
            sum: 0.0,
            batch_validator: BatchValidatorComponent::new(ctx.clone(), length),
            input_cache: WindowCacheComponent::new(ctx.clone(), length),
        };
    }
}

impl Component<Option<f64>, Option<f64>> for SumComponent {
    fn next(&mut self, value: Option<f64>) -> Option<f64> {
        self.input_cache.next(value);
        let first_value = self.input_cache.first_unwrapped();
        let last_value = self.input_cache.last_unwrapped();
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
