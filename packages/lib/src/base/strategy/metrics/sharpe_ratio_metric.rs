use crate::base::components::component_context::ComponentContext;

use super::{equity_metric::Equity, sharpe_ratio::compute_sharpe_ratio};

pub struct SharpeRatioMetricConfig {
    pub risk_free_rate: f64,
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

    pub fn next(&mut self, equity: &Equity) -> f64 {
        self.ctx.assert();

        let sharpe_ratio = compute_sharpe_ratio(
            equity.returns_mean,
            equity.returns_stdev,
            self.config.risk_free_rate,
        );

        return sharpe_ratio;
    }
}
