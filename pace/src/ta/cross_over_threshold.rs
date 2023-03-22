use super::cross::{cross_over, cross_under, CrossMode};
use crate::core::{context::Context, incremental::Incremental};

/// Similar to `CrossOver`, but the `threshold` is fixed and set on initialization.
pub struct CrossOverThreshold {
    pub ctx: Context,
    prev_value: Option<f64>,
    threshold: f64,
}

impl CrossOverThreshold {
    pub fn new(ctx: Context, threshold: f64) -> Self {
        return Self {
            ctx,
            prev_value: None,
            threshold,
        };
    }
}

impl Incremental<Option<f64>, bool> for CrossOverThreshold {
    fn next(&mut self, value: Option<f64>) -> bool {
        let cross = match (self.prev_value, value) {
            (Some(prev_value), Some(value)) => {
                cross_over(value, self.threshold, prev_value, self.threshold)
            }
            _ => false,
        };

        self.prev_value = value;

        return cross;
    }
}
