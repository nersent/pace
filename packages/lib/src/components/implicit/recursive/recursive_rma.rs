use crate::components::component_context::ComponentContext;

use super::recursive_ema::RecursiveEMA;

pub struct RecursiveRMA {
    pub length: usize,
    ctx: ComponentContext,
    ema: RecursiveEMA,
}

// Running Moving Average
// Used in RSI
impl RecursiveRMA {
    pub fn new(ctx: ComponentContext, length: usize) -> Self {
        assert!(length > 1, "RecursiveRMA must have a length larger than 1");
        return RecursiveRMA {
            length,
            ctx: ctx.clone(),
            ema: RecursiveEMA::with_alpha(ctx.clone(), length, 1.0 / length as f64),
        };
    }

    pub fn next(&mut self, value: Option<f64>) -> Option<f64> {
        self.ctx.assert();
        return self.ema.next(value);
    }
}
