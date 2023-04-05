use crate::core::{context::Context, incremental::Incremental};

/// Fixes NaN values by replacing them with the last non-NaN value.
pub struct FixNan {
    pub ctx: Context,
    last_non_nan_value: f64,
}

impl FixNan {
    pub fn new(ctx: Context) -> Self {
        return Self {
            ctx: ctx.clone(),
            last_non_nan_value: f64::NAN,
        };
    }
}

impl Incremental<f64, f64> for FixNan {
    fn next(&mut self, value: f64) -> f64 {
        if value.is_nan() {
            return self.last_non_nan_value;
        }

        self.last_non_nan_value = value;
        return value;
    }
}
