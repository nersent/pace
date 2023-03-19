use crate::{
    components::component::Component,
    statistics::stdev_component::StdevComponent,
    strategy::{
        metrics::common::max_run_up_percent, strategy_context::StrategyContext,
        trade::TradeDirection,
    },
};

use super::{
    cobra_metrics_component::{CobraMetrics, CobraMetricsComponent, CobraMetricsComponentConfig},
    common::{
        avg_losing_trade, avg_trade, avg_win_loss_ratio, avg_winning_trade, gross_profit_percent,
        max_drawdown_percent, net_profit_percent, percent_profitable, profit_factor, sharpe_ratio,
        sortino_ratio,
    },
    equity_metrics_component::{EquityMetrics, EquityMetricsComponent},
    performance_metrics_component::{PerformanceMetrics, PerformanceMetricsComponent},
    returns_component::ReturnsComponent,
    tradingview_metrics_component::{
        TradingViewMetrics, TradingViewMetricsComponent, TradingViewMetricsComponentConfig,
    },
};

/// Standalone version of `CobraMetricsComponent`.
pub struct WrappedCobraMetricsComponent {
    pub sctx: StrategyContext,
    pub config: CobraMetricsComponentConfig,
    pub data: CobraMetrics,
    equity_metrics: EquityMetricsComponent,
    performance_metrics: PerformanceMetricsComponent,
    cobra_metrics: CobraMetricsComponent,
}

impl WrappedCobraMetricsComponent {
    pub fn new(sctx: StrategyContext, config: CobraMetricsComponentConfig) -> Self {
        return Self {
            sctx: sctx.clone(),
            data: CobraMetrics::default(),
            equity_metrics: EquityMetricsComponent::new(sctx.clone()),
            performance_metrics: PerformanceMetricsComponent::new(sctx.clone()),
            cobra_metrics: CobraMetricsComponent::new(sctx.clone(), config),
            config,
        };
    }
}

impl Component<(), ()> for WrappedCobraMetricsComponent {
    fn next(&mut self, _: ()) {
        self.equity_metrics.next(());
        self.performance_metrics.next(&self.equity_metrics.data);
        self.cobra_metrics
            .next((&self.equity_metrics.data, &self.performance_metrics.data));
        self.data = self.cobra_metrics.data;
    }
}
