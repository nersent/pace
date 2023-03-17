use crate::components::{
    component::Component, component_context::ComponentContext,
    value_cache_component::ValueCacheComponent,
};

use super::{
    common::variance, mean_component::MeanComponent, welfords_var_component::WelfordsVarComponent,
};

// Variance.
///
/// For O(1) complexity, use `fast`. By default it's `false`.
///
/// Compared to `ta::var`, this component calculates variance based on entire history of values.
///
/// Not the same as PineScript `ta.var`.
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
