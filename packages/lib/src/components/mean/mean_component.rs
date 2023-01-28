use crate::{
    components::component_context::ComponentContext,
    ta::moving_average::ema_component::ExponentialMovingAverageComponent,
};

pub struct MeanComponent {
    ctx: ComponentContext,
    sum: f64,
    pub n: usize,
}

impl MeanComponent {
    pub fn new(ctx: ComponentContext) -> Self {
        return MeanComponent {
            ctx: ctx.clone(),
            sum: 0.0,
            n: 0,
        };
    }

    pub fn next(&mut self, value: f64) -> f64 {
        self.ctx.assert();
        self.sum += value;
        self.n += 1;
        return self.sum / self.n as f64;
    }
}
