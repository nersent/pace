use crate::{
    components::component_context::ComponentContext, strategy::strategy_utils::compute_sharpe_ratio,
};

use super::strategy_equity_metric::StrategyEquity;

pub struct StrategySharpeRatioMetricConfig {
    pub risk_free_rate: f64,
    pub multiplier: f64,
}

pub struct StrategySharpeRatioMetric {
    pub config: StrategySharpeRatioMetricConfig,
    ctx: ComponentContext,
}

impl StrategySharpeRatioMetric {
    pub fn new(ctx: ComponentContext, config: StrategySharpeRatioMetricConfig) -> Self {
        return StrategySharpeRatioMetric {
            ctx: ctx.clone(),
            config,
        };
    }

    pub fn next(&mut self, equity: StrategyEquity) -> f64 {
        self.ctx.assert();

        let sharpe_ratio = compute_sharpe_ratio(
            equity.returns_mean,
            equity.returns_stdev,
            self.config.risk_free_rate,
        ) * self.config.multiplier;

        return sharpe_ratio;
    }
}
