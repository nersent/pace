use crate::components::component_context::ComponentContext;

use super::cross::{cross_over, cross_under, CrossMode};

pub struct CrossUnderThresholdComponent {
    pub ctx: ComponentContext,
    prev_value: Option<f64>,
    threshold: f64,
}

impl CrossUnderThresholdComponent {
    pub fn new(ctx: ComponentContext, threshold: f64) -> Self {
        return CrossUnderThresholdComponent {
            ctx,
            prev_value: None,
            threshold,
        };
    }

    pub fn next(&mut self, value: Option<f64>) -> bool {
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
