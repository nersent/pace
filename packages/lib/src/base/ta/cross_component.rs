use crate::base::components::component_context::ComponentContext;

use super::cross::{compute_cross_over, compute_cross_under, CrossMode};

pub struct CrossComponent {
    ctx: ComponentContext,
    prev_a_value: Option<f64>,
    prev_b_value: Option<f64>,
    mode: CrossMode,
}

impl CrossComponent {
    pub fn new(ctx: ComponentContext, mode: CrossMode) -> Self {
        return CrossComponent {
            ctx,
            prev_a_value: None,
            prev_b_value: None,
            mode,
        };
    }

    pub fn next(&mut self, a: Option<f64>, b: Option<f64>) -> bool {
        self.ctx.assert();

        let cross = match (self.prev_a_value, self.prev_b_value, a, b) {
            (Some(prev_a), Some(prev_b), Some(a), Some(b)) => {
                if self.mode == CrossMode::Over {
                    compute_cross_over(a, b, prev_a, prev_b)
                } else {
                    compute_cross_under(a, b, prev_a, prev_b)
                }
            }
            _ => false,
        };

        self.prev_a_value = a;
        self.prev_b_value = b;

        return cross;
    }
}
