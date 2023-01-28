use crate::{
    components::component_context::ComponentContext,
    strategy::strategy_utils::{compute_omega_ratio, compute_sharpe_ratio},
};

use super::strategy_equity_metric::StrategyEquity;

pub struct StrategyOmegaRatioMetricConfig {
    pub risk_free_rate: f64,
}

pub struct StrategyOmegaRatioMetric {
    pub config: StrategyOmegaRatioMetricConfig,
    ctx: ComponentContext,
    positive_returns_sum: f64,
    negative_returns_sum: f64,
}

impl StrategyOmegaRatioMetric {
    pub fn new(ctx: ComponentContext, config: StrategyOmegaRatioMetricConfig) -> Self {
        return StrategyOmegaRatioMetric {
            ctx: ctx.clone(),
            config,
            positive_returns_sum: 0.0,
            negative_returns_sum: 0.0,
        };
    }

    pub fn next(&mut self, equity: StrategyEquity) -> f64 {
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
