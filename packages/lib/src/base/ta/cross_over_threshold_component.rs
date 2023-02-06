use crate::base::components::component_context::ComponentContext;

use super::cross::{compute_cross_over, compute_cross_under, CrossMode};

pub struct CrossOverThresholdComponent {
    ctx: ComponentContext,
    prev_value: Option<f64>,
    threshold: f64,
}

impl CrossOverThresholdComponent {
    pub fn new(ctx: ComponentContext, threshold: f64) -> Self {
        return CrossOverThresholdComponent {
            ctx,
            prev_value: None,
            threshold,
        };
    }

    pub fn next(&mut self, value: Option<f64>) -> bool {
        self.ctx.assert();

        let cross = match (self.prev_value, value) {
            (Some(prev_value), Some(value)) => {
                compute_cross_over(value, self.threshold, prev_value, self.threshold)
            }
            _ => false,
        };

        self.prev_value = value;

        return cross;
    }
}
