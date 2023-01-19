use crate::components::component_context::ComponentContext;

use super::{recursive_position::RecursivePosition, recursive_sma::RecursiveSMA};

pub struct RecursiveEMA {
    pub alpha: f64,
    pub length: usize,
    ctx: ComponentContext,
    sma: RecursiveSMA,
    position: RecursivePosition,
    prev_value: Option<f64>,
}

// Exponential Moving Average
impl RecursiveEMA {
    pub fn new(ctx: ComponentContext, length: usize) -> Self {
        return RecursiveEMA::with_alpha(ctx, length, 2.0 / (length as f64 + 1.0));
    }

    pub fn with_alpha(ctx: ComponentContext, length: usize, alpha: f64) -> Self {
        assert!(length > 1, "RecursiveEMA must have a length larger than 1");
        return RecursiveEMA {
            length,
            alpha,
            ctx: ctx.clone(),
            position: RecursivePosition::new(ctx.clone()),
            sma: RecursiveSMA::new(ctx.clone(), length),
            prev_value: None,
        };
    }

    pub fn next(&mut self, value: Option<f64>) -> Option<f64> {
        self.ctx.assert();
        let current_index = self.position.next();
        if current_index < self.length - 1 {
            self.sma.next(value);
            return None;
        }
        match self.prev_value {
            Some(prev_value) => {
                let ema = self.alpha * value.unwrap() + (1.0 - self.alpha) * prev_value;
                self.prev_value = Some(ema);
                return self.prev_value;
            }
            None => {
                self.prev_value = self.sma.next(value);
                return self.prev_value;
            }
        }
    }
}
