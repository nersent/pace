use crate::core::{context::Context, incremental::Incremental};

/// Fixes NaN values by replacing them with the last non-NaN value.
pub struct FixNan {
    pub ctx: Context,
    last_non_nan_value: Option<f64>,
}

impl FixNan {
    pub fn new(ctx: Context) -> Self {
        return Self {
            ctx: ctx.clone(),
            last_non_nan_value: None,
        };
    }
}

impl Incremental<Option<f64>, Option<f64>> for FixNan {
    fn next(&mut self, value: Option<f64>) -> Option<f64> {
        match value {
            Some(value) => {
                self.last_non_nan_value = Some(value);
                return Some(value);
            }
            None => {
                return self.last_non_nan_value;
            }
        }
    }
}
