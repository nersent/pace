use crate::components::{component::Component, component_context::ComponentContext};

use super::{stats::stdev_from_var, var_component::VarComponent};

pub struct StdevComponent {
    pub ctx: ComponentContext,
    pub fast: bool,
    variance: VarComponent,
}

impl StdevComponent {
    pub fn build(ctx: ComponentContext, fast: bool) -> Self {
        return Self {
            ctx: ctx.clone(),
            fast,
            variance: VarComponent::build(ctx.clone(), fast),
        };
    }

    pub fn new(ctx: ComponentContext) -> Self {
        return Self::build(ctx, false);
    }

    pub fn fast(ctx: ComponentContext) -> Self {
        return Self::build(ctx, true);
    }
}

impl Component<f64, f64> for StdevComponent {
    fn next(&mut self, value: f64) -> f64 {
        return stdev_from_var(self.variance.next(value));
    }
}
