use super::cross::{cross_over, cross_under, CrossMode};
use crate::components::component_context::ComponentContext;

/// Similar to `CrossOverThresholdComponent` and `CrossUnderThresholdComponent`, but there is only one `threshold` for every `CrossMode`.
pub struct CrossThresholdComponent {
    pub ctx: ComponentContext,
    prev_value: Option<f64>,
    threshold: f64,
}

impl CrossThresholdComponent {
    pub fn new(ctx: ComponentContext, threshold: f64) -> Self {
        return CrossThresholdComponent {
            ctx,
            prev_value: None,
            threshold,
        };
    }

    pub fn next(&mut self, value: Option<f64>) -> Option<CrossMode> {
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
