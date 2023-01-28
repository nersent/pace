use crate::{
    components::{
        component_context::ComponentContext, variance::variance_component::VarianceComponent,
    },
    ta::moving_average::ema_component::ExponentialMovingAverageComponent,
};

pub struct StandardDeviationComponent {
    ctx: ComponentContext,
    variance: VarianceComponent,
}

// Computes standard deviation using Welford's online algorithm
impl StandardDeviationComponent {
    pub fn new(ctx: ComponentContext) -> Self {
        return StandardDeviationComponent {
            ctx: ctx.clone(),
            variance: VarianceComponent::new(ctx.clone()),
        };
    }

    pub fn next(&mut self, value: f64) -> f64 {
        self.ctx.assert();
        let variance = self.variance.next(value);
        return variance.map(|v| v.sqrt()).unwrap_or(0.0);
    }
}
