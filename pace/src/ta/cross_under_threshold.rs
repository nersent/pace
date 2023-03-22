use crate::core::{context::Context, incremental::Incremental};

use super::cross::{cross_over, cross_under, CrossMode};

/// Similar to `CrossOver`, but the `threshold` is fixed and set on initialization.
pub struct CrossUnderThreshold {
    pub ctx: Context,
    prev_value: Option<f64>,
    threshold: f64,
}

impl CrossUnderThreshold {
    pub fn new(ctx: Context, threshold: f64) -> Self {
        return Self {
            ctx,
            prev_value: None,
            threshold,
        };
    }
}

impl Incremental<Option<f64>, bool> for CrossUnderThreshold {
    fn next(&mut self, value: Option<f64>) -> bool {
        let cross = match (self.prev_value, value) {
            (Some(prev_value), Some(value)) => {
                cross_under(value, self.threshold, prev_value, self.threshold)
            }
            _ => false,
        };

        self.prev_value = value;

        return cross;
    }
}
