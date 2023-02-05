use crate::{
    components::{
        batch_validator::recursive_batch_validator::RecursiveBatchValidator,
        component_context::ComponentContext, lifo::recursive_lifo::RecursiveLIFO,
    },
    math::comparison::FloatComparison,
    ta::moving_average::{
        rma_component::RunningMovingAverageComponent, sma_component::SimpleMovingAverageComponent,
    },
};

pub struct SymmetricallyWeightedMovingAverageComponent {
    length: usize,
    ctx: ComponentContext,
    value_lifo: RecursiveLIFO,
    batch_validator: RecursiveBatchValidator,
}

static WEIGHTS: [f64; 4] = [1.0 / 6.0, 2.0 / 6.0, 2.0 / 6.0, 1.0 / 6.0];

impl SymmetricallyWeightedMovingAverageComponent {
    pub fn new(ctx: ComponentContext) -> Self {
        let length = 4;
        return SymmetricallyWeightedMovingAverageComponent {
            ctx: ctx.clone(),
            length,
            value_lifo: RecursiveLIFO::new(ctx.clone(), length),
            batch_validator: RecursiveBatchValidator::new(ctx.clone(), length),
        };
    }

    pub fn next(&mut self, value: Option<f64>) -> Option<f64> {
        self.ctx.assert();

        let (first_value, last_value, is_filled) = self.value_lifo.next(value);
        let is_valid = self.batch_validator.next(value);

        if !self.ctx.get().at_length(self.length) || !is_valid {
            return None;
        }

        let values = self.value_lifo.values_with_first();
        let mut swma = 0.0;

        for i in 0..values.len() {
            let value = values[values.len() - 1 - i].unwrap();
            let weight = WEIGHTS[i];
            let weighted_value = value * weight;
            swma += weighted_value;
        }

        return Some(swma);
    }
}
