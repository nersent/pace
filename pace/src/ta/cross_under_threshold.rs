use crate::core::{context::Context, incremental::Incremental};

use super::cross::{cross_over, cross_under, CrossMode};

/// Similar to `CrossOver`, but the `threshold` is fixed and set on initialization.
pub struct CrossUnderThreshold {
    pub ctx: Context,
    prev_value: f64,
    threshold: f64,
}

impl CrossUnderThreshold {
    pub fn new(ctx: Context, threshold: f64) -> Self {
        return Self {
            ctx,
            prev_value: f64::NAN,
            threshold,
        };
    }
}

impl Incremental<f64, bool> for CrossUnderThreshold {
    fn next(&mut self, value: f64) -> bool {
        let cross = !value.is_nan()
            && !self.prev_value.is_nan()
            && cross_under(value, self.threshold, self.prev_value, self.threshold);

        self.prev_value = value;

        return cross;
    }
}
