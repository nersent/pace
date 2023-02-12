use crate::base::components::component_context::ComponentContext;

use super::welfords_variance_component::WelfordsVarianceComponent;

pub struct WelfordsStandardDeviationComponent {
    ctx: ComponentContext,
    variance: WelfordsVarianceComponent,
}

// Computes standard deviation using Welford's online algorithm
impl WelfordsStandardDeviationComponent {
    pub fn new(ctx: ComponentContext) -> Self {
        return WelfordsStandardDeviationComponent {
            ctx: ctx.clone(),
            variance: WelfordsVarianceComponent::new(ctx.clone()),
        };
    }

    pub fn next(&mut self, value: f64) -> f64 {
        // self.ctx.on_next();
        let variance = self.variance.next(value);
        return variance.map(|v| v.sqrt()).unwrap_or(0.0);
    }
}
