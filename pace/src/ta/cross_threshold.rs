use crate::core::{context::Context, incremental::Incremental};

use super::cross::{cross_over, cross_under, CrossMode};

/// Similar to `CrossOverThreshold` and `CrossUnderThreshold`, but there is only one `threshold` for every `CrossMode`.
pub struct CrossThreshold {
    pub ctx: Context,
    prev_value: Option<f64>,
    threshold: f64,
}

impl CrossThreshold {
    pub fn new(ctx: Context, threshold: f64) -> Self {
        return Self {
            ctx,
            prev_value: None,
            threshold,
        };
    }
}

impl Incremental<Option<f64>, Option<CrossMode>> for CrossThreshold {
    fn next(&mut self, value: Option<f64>) -> Option<CrossMode> {
        let cross = match (self.prev_value, value) {
            (Some(prev_value), Some(value)) => {
                if cross_over(value, self.threshold, prev_value, self.threshold) {
                    Some(CrossMode::Over)
                } else if cross_under(value, self.threshold, prev_value, self.threshold) {
                    Some(CrossMode::Under)
                } else {
                    None
                }
            }
            _ => None,
        };

        self.prev_value = value;

        return cross;
    }
}
