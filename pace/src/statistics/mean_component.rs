use crate::components::{component::Component, component_context::ComponentContext};

/// Calculates mean for all history of values.
pub struct MeanComponent {
    pub ctx: ComponentContext,
    sum: f64,
    n: usize,
}

impl MeanComponent {
    pub fn new(ctx: ComponentContext) -> Self {
        return Self {
            ctx: ctx.clone(),
            sum: 0.0,
            n: 0,
        };
    }
}

impl Component<f64, f64> for MeanComponent {
    fn next(&mut self, value: f64) -> f64 {
        self.sum += value;
        self.n += 1;
        return self.sum / self.n as f64;
    }
}
