use crate::core::{context::Context, incremental::Incremental};

pub struct PrevChange {
    pub ctx: Context,
    prev_value: f64,
}

impl PrevChange {
    pub fn new(ctx: Context) -> Self {
        return Self::with_initial(ctx, f64::NAN);
    }

    pub fn with_initial(ctx: Context, initial_value: f64) -> Self {
        return Self {
            ctx: ctx.clone(),
            prev_value: initial_value,
        };
    }
}

impl Incremental<f64, f64> for PrevChange {
    fn next(&mut self, value: f64) -> f64 {
        let diff = value - self.prev_value;
        self.prev_value = value;
        return diff;
    }
}
