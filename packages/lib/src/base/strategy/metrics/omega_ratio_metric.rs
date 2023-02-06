use crate::base::components::component_context::ComponentContext;

use super::{equity_metric::Equity, omega_ratio::compute_omega_ratio};

pub struct OmegaRatioMetricConfig {
    pub risk_free_rate: f64,
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

    pub fn next(&mut self, equity: &Equity) -> f64 {
        self.ctx.assert();

        let returns = equity.returns;

        if returns > 0.0 {
            self.positive_returns_sum += returns;
        } else {
            self.negative_returns_sum += returns.abs();
        }

        return compute_omega_ratio(self.positive_returns_sum, self.negative_returns_sum);
    }
}
