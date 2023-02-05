use crate::base::components::{
    common::{
        batch_validator_component::BatchValidatorComponent,
        fixed_value_cache_component::FixedValueCacheComponent,
    },
    component_context::ComponentContext,
};

pub struct WeightedMovingAverageComponent {
    pub length: usize,
    ctx: ComponentContext,
    input_cache: FixedValueCacheComponent,
    batch_validator: BatchValidatorComponent,
}

impl WeightedMovingAverageComponent {
    pub fn new(ctx: ComponentContext, length: usize) -> Self {
        assert!(
            length > 0,
            "WeightedMovingAverageComponent must have a length of at least 1"
        );
        return WeightedMovingAverageComponent {
            ctx: ctx.clone(),
            length,
            input_cache: FixedValueCacheComponent::new(ctx.clone(), length),
            batch_validator: BatchValidatorComponent::new(ctx.clone(), length),
        };
    }

    pub fn next(&mut self, value: Option<f64>) -> Option<f64> {
        self.ctx.assert();

        self.input_cache.next(value);
        let is_valid = self.batch_validator.next(value);

        if !self.ctx.get().at_length(self.length) || !is_valid {
            return None;
        }

        let values = self.input_cache.all();

        let (sum, norm) = values
            .iter()
            .rev()
            .enumerate()
            .fold((0.0, 0.0), |acc, (i, value)| {
                let value = value.unwrap();
                let weight = ((self.length - i) * self.length) as f64;
                let weighted_value = value * weight;
                (acc.0 + weighted_value, acc.1 + weight)
            });

        let wma = sum / norm;

        return Some(wma);
    }
}
