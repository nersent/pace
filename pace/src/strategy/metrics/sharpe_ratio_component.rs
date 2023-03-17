use crate::components::{
    component::Component, component_context::ComponentContext, component_default::ComponentDefault,
};

use super::common::sharpe_ratio;

pub struct SharpeRatioComponentConfig {
    pub risk_free_rate: f64,
}

impl Default for SharpeRatioComponentConfig {
    fn default() -> Self {
        return Self {
            risk_free_rate: 0.0,
        };
    }
}

pub struct SharpeRatioComponent {
    pub config: SharpeRatioComponentConfig,
    pub ctx: ComponentContext,
    pub positive_returns_sum: f64,
    pub negative_returns_sum: f64,
}

impl SharpeRatioComponent {
    pub fn new(ctx: ComponentContext, config: SharpeRatioComponentConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            config,
            positive_returns_sum: 0.0,
            negative_returns_sum: 0.0,
        };
    }
}

impl Component<(f64, f64), f64> for SharpeRatioComponent {
    fn next(&mut self, (returns_mean, returns_stdev): (f64, f64)) -> f64 {
        return sharpe_ratio(returns_mean, returns_stdev, self.config.risk_free_rate);
    }
}
