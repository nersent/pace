use crate::{
    components::component_context::ComponentContext,
    ta::moving_average::ema_component::ExponentialMovingAverageComponent,
};

pub struct VarianceComponent {
    ctx: ComponentContext,
    mean: f64,
    deviation: f64,
    pub n: usize,
}

// Computes variance using Welford's online algorithm
impl VarianceComponent {
    pub fn new(ctx: ComponentContext) -> Self {
        return VarianceComponent {
            ctx: ctx.clone(),
            mean: 0.0,
            deviation: 0.0,
            n: 0,
        };
    }

    pub fn next(&mut self, value: f64) -> Option<f64> {
        self.ctx.assert();

        self.n += 1;

        let delta = value - self.mean;

        self.mean += delta / (self.n as f64);

        self.deviation += delta * (value - self.mean);

        if self.n <= 1 {
            return None;
        }

        let variance = self.deviation / (self.n as f64 - 1.0);

        return Some(variance);
    }
}
