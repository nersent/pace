use crate::components::{
    component_context::ComponentContext, implicit::recursive::recursive_lifo::RecursiveLIFO,
};

use super::recursive_batch_validator::RecursiveBatchValidator;

pub struct RecursiveSMA {
    pub length: usize,
    ctx: ComponentContext,
    _length_f64: f64,
    sum: f64,
    lifo: RecursiveLIFO,
    batch_validator: RecursiveBatchValidator,
}

// Simple Moving Average
impl RecursiveSMA {
    pub fn new(ctx: ComponentContext, length: usize) -> Self {
        assert!(length > 1, "RecursiveSMA must have a length larger than 1");
        return RecursiveSMA {
            length,
            ctx: ctx.clone(),
            _length_f64: length as f64,
            sum: 0.0,
            lifo: RecursiveLIFO::new(ctx.clone(), length),
            batch_validator: RecursiveBatchValidator::new(ctx.clone(), length),
        };
    }

    pub fn next(&mut self, value: Option<f64>) -> Option<f64> {
        self.ctx.assert();
        let is_valid = self.batch_validator.next(value);
        let (first_value, last_value, is_filled) = self.lifo.next(value);
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
