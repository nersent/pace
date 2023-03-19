use crate::{
    components::component::Component,
    statistics::stdev_component::StdevComponent,
    strategy::{
        metrics::common::max_run_up_percent, strategy_context::StrategyContext,
        trade::TradeDirection,
    },
};

use super::{
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

/// Standalone version of `TradingViewMetricsComponent`.
pub struct WrappedTradingViewMetricsComponent {
    pub sctx: StrategyContext,
    pub config: TradingViewMetricsComponentConfig,
    pub data: TradingViewMetrics,
    equity_metrics: EquityMetricsComponent,
    performance_metrics: PerformanceMetricsComponent,
    tradingview_metrics: TradingViewMetricsComponent,
}

impl WrappedTradingViewMetricsComponent {
    pub fn new(sctx: StrategyContext, config: TradingViewMetricsComponentConfig) -> Self {
        return Self {
            sctx: sctx.clone(),
            data: TradingViewMetrics::default(sctx.initial_capital),
            config,
            equity_metrics: EquityMetricsComponent::new(sctx.clone()),
            performance_metrics: PerformanceMetricsComponent::new(sctx.clone()),
            tradingview_metrics: TradingViewMetricsComponent::new(sctx.clone(), config),
        };
    }
}

impl Component<(), ()> for WrappedTradingViewMetricsComponent {
    fn next(&mut self, _: ()) {
        self.equity_metrics.next(());
        self.performance_metrics.next(&self.equity_metrics.data);
        self.tradingview_metrics
            .next((&self.equity_metrics.data, &self.performance_metrics.data));
        self.data = self.tradingview_metrics.data.clone();
    }
}
