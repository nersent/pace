use crate::core::{context::Context, incremental::Incremental};

pub struct Cum {
    pub ctx: Context,
    sum: f64,
}

impl Cum {
    pub fn new(ctx: Context) -> Self {
        Self {
            ctx: ctx.clone(),
            sum: 0.0,
        }
    }
}

impl Incremental<f64, f64> for Cum {
    fn next(&mut self, value: f64) -> f64 {
        if value.is_nan() {
            return f64::NAN;
        }
        self.sum += value;
        return self.sum;
    }
}
