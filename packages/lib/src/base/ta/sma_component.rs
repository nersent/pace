use crate::base::components::{
    common::{
        batch_validator_component::BatchValidatorComponent,
        fixed_value_cache_component::FixedValueCacheComponent,
    },
    component_context::ComponentContext,
};

pub struct SimpleMovingAverageComponent {
    pub length: usize,
    ctx: ComponentContext,
    _length_f64: f64,
    sum: f64,
    input_cache: FixedValueCacheComponent,
    batch_validator: BatchValidatorComponent,
}

impl SimpleMovingAverageComponent {
    pub fn new(ctx: ComponentContext, length: usize) -> Self {
        assert!(length > 0, "RecursiveSMA must have a length larger than 0");
        return SimpleMovingAverageComponent {
            length,
            ctx: ctx.clone(),
            _length_f64: length as f64,
            sum: 0.0,
            input_cache: FixedValueCacheComponent::new(ctx.clone(), length),
            batch_validator: BatchValidatorComponent::new(ctx.clone(), length),
        };
    }

    pub fn next(&mut self, value: Option<f64>) -> Option<f64> {
        self.ctx.on_next();
        if self.length == 1 {
            return value;
        }
        self.input_cache.next(value);
        let is_valid = self.batch_validator.next(value);
        let is_filled = self.input_cache.is_filled();
        let first_value = self.input_cache.first();
        let last_value = self.input_cache.last();
        let mut mean: Option<f64> = None;
        if let Some(last_value) = last_value {
            self.sum += last_value;
        }
        if is_filled && is_valid {
            mean = Some(self.sum / self._length_f64);
        }
        if let Some(first_value) = first_value {
            self.sum -= first_value;
        }
        return mean;
    }
}
