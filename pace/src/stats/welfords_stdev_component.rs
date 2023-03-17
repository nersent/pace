use crate::components::{component::Component, component_context::ComponentContext};

use super::{stats::stdev_from_var, welfords_var_component::WelfordsVarComponent};

/// Calculates standard deviation using Welford's online algorithm.
pub struct WelfordsStdevComponent {
    pub ctx: ComponentContext,
    variance: WelfordsVarComponent,
}

impl WelfordsStdevComponent {
    pub fn new(ctx: ComponentContext) -> Self {
        return Self {
            ctx: ctx.clone(),
            variance: WelfordsVarComponent::new(ctx.clone()),
        };
    }
}

impl Component<f64, f64> for WelfordsStdevComponent {
    fn next(&mut self, value: f64) -> f64 {
        return stdev_from_var(self.variance.next(value));
    }
}
