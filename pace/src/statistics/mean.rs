use crate::core::{context::Context, incremental::Incremental};

/// Calculates mean for all history of values.
pub struct Mean {
    pub ctx: Context,
    sum: f64,
    n: usize,
}

impl Mean {
    pub fn new(ctx: Context) -> Self {
        return Self {
            ctx: ctx.clone(),
            sum: 0.0,
            n: 0,
        };
    }
}

impl Incremental<f64, f64> for Mean {
    fn next(&mut self, value: f64) -> f64 {
        self.sum += value;
        self.n += 1;
        return self.sum / self.n as f64;
    }
}
