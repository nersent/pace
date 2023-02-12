use crate::base::components::{
    component_context::ComponentContext, component_default::ComponentDefault,
};

use super::{equity_metric::Equity, sharpe_ratio::compute_sharpe_ratio};

pub struct SharpeRatioMetricConfig {
    pub risk_free_rate: f64,
}

impl ComponentDefault for SharpeRatioMetricConfig {
    fn default(ctx: ComponentContext) -> Self {
        return Self {
            risk_free_rate: 0.0,
        };
    }
}

pub struct SharpeRatioMetric {
    pub config: SharpeRatioMetricConfig,
    ctx: ComponentContext,
}

impl SharpeRatioMetric {
    pub fn new(ctx: ComponentContext, config: SharpeRatioMetricConfig) -> Self {
        return SharpeRatioMetric {
            ctx: ctx.clone(),
            config,
        };
    }

    pub fn next(&mut self, returns_mean: f64, returns_stdev: f64) -> f64 {
        // self.ctx.assert();

        let sharpe_ratio =
            compute_sharpe_ratio(returns_mean, returns_stdev, self.config.risk_free_rate);

        return sharpe_ratio;
    }
}
