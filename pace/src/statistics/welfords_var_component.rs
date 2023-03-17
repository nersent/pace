use crate::components::{component::Component, component_context::ComponentContext};

/// Calculates variance using Welford's online algorithm. Has O(1) complexity.
pub struct WelfordsVarComponent {
    pub ctx: ComponentContext,
    n: usize,
    mean: f64,
    deviation: f64,
}

impl WelfordsVarComponent {
    pub fn new(ctx: ComponentContext) -> Self {
        return Self {
            ctx: ctx.clone(),
            mean: 0.0,
            deviation: 0.0,
            n: 0,
        };
    }
}

impl Component<f64, f64> for WelfordsVarComponent {
    fn next(&mut self, value: f64) -> f64 {
        self.n += 1;

        let delta = value - self.mean;

        self.mean += delta / (self.n as f64);
        self.deviation += delta * (value - self.mean);

        if self.n <= 1 {
            return 0.0;
        }

        return self.deviation / (self.n as f64 - 1.0);
    }
}
