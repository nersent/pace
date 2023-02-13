use crate::base::{
    components::{component_context::ComponentContext, component_default::ComponentDefault},
    strategy::strategy_context::StrategyMetrics,
};

use super::omega_ratio::compute_omega_ratio;

pub struct MaxDrawdownMetric {
    ctx: ComponentContext,
    max_dd: f64,
    peak: f64,
}

impl MaxDrawdownMetric {
    pub fn new(ctx: ComponentContext, initial_value: f64) -> Self {
        return Self {
            ctx: ctx.clone(),
            max_dd: 0.0,
            peak: initial_value,
        };
    }

    pub fn next(&mut self, value: f64) -> f64 {
        self.ctx.on_next();

        if value > self.peak {
            self.peak = value;
        }

        let dd = (self.peak - value);
        if dd > self.max_dd {
            self.max_dd = dd;
        }

        return self.max_dd;
    }
}
