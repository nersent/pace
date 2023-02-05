use crate::{
    components::{
        batch_validator::recursive_batch_validator::RecursiveBatchValidator,
        component_context::ComponentContext, lifo::recursive_lifo::RecursiveLIFO,
        value_cache::fixed_value_cache_component::FixedValueCacheComponent,
    },
    math::comparison::FloatComparison,
    ta::moving_average::{
        rma_component::RunningMovingAverageComponent, sma_component::SimpleMovingAverageComponent,
    },
};

pub struct SymmetricallyWeightedMovingAverageComponent {
    length: usize,
    ctx: ComponentContext,
    input_cache: FixedValueCacheComponent,
    batch_validator: RecursiveBatchValidator,
}

static WEIGHTS: [f64; 4] = [1.0 / 6.0, 2.0 / 6.0, 2.0 / 6.0, 1.0 / 6.0];

impl SymmetricallyWeightedMovingAverageComponent {
    pub fn new(ctx: ComponentContext) -> Self {
        let length = 4;
        return SymmetricallyWeightedMovingAverageComponent {
            ctx: ctx.clone(),
            length,
            input_cache: FixedValueCacheComponent::new(ctx.clone(), length),
            batch_validator: RecursiveBatchValidator::new(ctx.clone(), length),
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
        let mut swma = 0.0;

        let swma = values.iter().enumerate().fold(0.0, |acc, (i, value)| {
            let value = value.unwrap();
            let weight = WEIGHTS[i];
            let weighted_value = value * weight;
            acc + weighted_value
        });

        return Some(swma);
    }
}
