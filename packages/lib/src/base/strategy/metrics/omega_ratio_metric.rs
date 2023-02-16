use crate::base::{
    components::{component_context::ComponentContext, component_default::ComponentDefault},
    strategy::strategy_context::StrategyMetrics,
};

use super::omega_ratio::compute_omega_ratio;

pub struct OmegaRatioMetricConfig {
    pub risk_free_rate: f64,
}

impl ComponentDefault for OmegaRatioMetricConfig {
    fn default(ctx: ComponentContext) -> Self {
        return Self {
            risk_free_rate: 0.0,
        };
    }
}

pub struct OmegaRatioMetric {
    pub config: OmegaRatioMetricConfig,
    ctx: ComponentContext,
    positive_returns_sum: f64,
    negative_returns_sum: f64,
}

impl OmegaRatioMetric {
    pub fn new(ctx: ComponentContext, config: OmegaRatioMetricConfig) -> Self {
        return OmegaRatioMetric {
            ctx: ctx.clone(),
            config,
            positive_returns_sum: 0.0,
            negative_returns_sum: 0.0,
        };
    }

    pub fn next(&mut self, returns: f64) -> f64 {
        self.ctx.on_next();

        if returns > 0.0 {
            self.positive_returns_sum += returns;
        } else {
            self.negative_returns_sum += returns.abs();
        }

        return compute_omega_ratio(self.positive_returns_sum, self.negative_returns_sum, 0.0);
    }
}
