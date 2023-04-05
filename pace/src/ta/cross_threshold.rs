use crate::core::{context::Context, incremental::Incremental};

use super::cross::{cross_over, cross_under, CrossMode};

/// Similar to `CrossOverThreshold` and `CrossUnderThreshold`, but there is only one `threshold` for every `CrossMode`.
pub struct CrossThreshold {
    pub ctx: Context,
    prev_value: f64,
    threshold: f64,
}

impl CrossThreshold {
    pub fn new(ctx: Context, threshold: f64) -> Self {
        return Self {
            ctx,
            prev_value: f64::NAN,
            threshold,
        };
    }
}

impl Incremental<f64, Option<CrossMode>> for CrossThreshold {
    fn next(&mut self, value: f64) -> Option<CrossMode> {
        let mut mode = None;

        if !self.prev_value.is_nan() && !value.is_nan() {
            if cross_over(value, self.threshold, self.prev_value, self.threshold) {
                mode = Some(CrossMode::Over)
            } else if cross_under(value, self.threshold, self.prev_value, self.threshold) {
                mode = Some(CrossMode::Under)
            }
        }

        self.prev_value = value;

        return mode;
    }
}
