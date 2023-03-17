use crate::components::{
    component::Component, component_context::ComponentContext,
    value_cache_component::ValueCacheComponent,
};

use super::{
    mean_component::MeanComponent, stats::variance, welfords_var_component::WelfordsVarComponent,
};

pub struct VarComponent {
    pub ctx: ComponentContext,
    pub fast: bool,
    input_cache: ValueCacheComponent<f64>,
    welfords_var: WelfordsVarComponent,
    mean: MeanComponent,
}

impl VarComponent {
    pub fn build(ctx: ComponentContext, fast: bool) -> Self {
        return Self {
            ctx: ctx.clone(),
            fast,
            welfords_var: WelfordsVarComponent::new(ctx.clone()),
            mean: MeanComponent::new(ctx.clone()),
            input_cache: ValueCacheComponent::new(ctx.clone()),
        };
    }

    pub fn new(ctx: ComponentContext) -> Self {
        return Self::build(ctx, false);
    }

    pub fn fast(ctx: ComponentContext) -> Self {
        return Self::build(ctx, true);
    }
}

impl Component<f64, f64> for VarComponent {
    fn next(&mut self, value: f64) -> f64 {
        if self.fast {
            return self.welfords_var.next(value);
        }

        self.input_cache.next(value);

        let mean = self.mean.next(value);
        let values = self.input_cache.all();

        return variance(values, mean);
    }
}
