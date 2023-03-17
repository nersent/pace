use crate::{
    components::component::Component,
    strategy::{
        metrics::metrics::max_run_up_percent, strategy_context::StrategyContext,
        trade::TradeDirection,
    },
};

use super::{
    equity_metrics_component::EquityMetrics,
    metrics::{
        avg_losing_trade, avg_trade, avg_win_loss_ratio, avg_winning_trade, percent_profitable,
        profit_factor,
    },
};

pub struct PerformanceMetrics {
    /// The overall profit or loss. In TradingView `strategy.netprofit`
    pub net_profit: f64,
    /// Current unrealized profit or loss for all open positions. In TradingView `strategy.openprofit`
    pub open_profit: f64,
    /// Total value of all completed winning trades. In TradingView `strategy.grossprofit`
    pub gross_profit: f64,
    /// Total value of all completed losing trades. In TradingView `strategy.grossloss`
    pub gross_loss: f64,
    /// Total number of closed tradesIn TradingView `strategy.closedtrades`
    pub closed_trades: usize,
    /// Total number of winning tradesIn TradingView `strategy.wintrades`
    pub winning_trades: usize,
    /// Total number of losing tradesIn TradingView `strategy.losstrades
    pub losing_trades: usize,
    /// The overall profit or loss for long positions.
    pub long_net_profit: f64,
    /// The overall profit or loss for short positions.
    pub short_net_profit: f64,
    // Maximum equity drawdown value for the whole trading interval. In TradingView `strategy.max_drawdown`
    pub max_drawdown: f64,
    /// Maximum equity run-up value for the whole trading interval. In TradingView `strategy.max_runup`
    pub max_run_up: f64,
    /// The gross profit divided by the number of winning trades.
    pub avg_winning_trade: f64,
    /// The gross loss divided by the number of losing trades.
    pub avg_losing_trade: f64,
    /// The sum of money gained or lost by the average trade.
    pub avg_trade: f64,
    /// The average value of how many currency units you win for every unit you lose.
    pub avg_winning_losing_trade_ratio: f64,
    /// The amount of money made for every unit of money it lost.
    pub profit_factor: f64,
    /// The percentage of winning trades generated by a strategy.
    pub profitable: f64,
}

impl PerformanceMetrics {
    pub fn default() -> Self {
        return Self {
            net_profit: 0.0,
            open_profit: 0.0,
            closed_trades: 0,
            gross_loss: 0.0,
            gross_profit: 0.0,
            long_net_profit: 0.0,
            losing_trades: 0,
            short_net_profit: 0.0,
            winning_trades: 0,
            max_drawdown: 0.0,
            max_run_up: 0.0,
            avg_winning_trade: 0.0,
            avg_losing_trade: 0.0,
            avg_trade: 0.0,
            avg_winning_losing_trade_ratio: 0.0,
            profit_factor: 0.0,
            profitable: 0.0,
        };
    }
}

/// Generic strategy metrics.
pub struct PerformanceMetricsComponent {
    pub sctx: StrategyContext,
    pub data: PerformanceMetrics,
}

impl PerformanceMetricsComponent {
    pub fn new(sctx: StrategyContext) -> Self {
        let state = sctx.state();
        return Self {
            sctx: sctx.clone(),
            data: PerformanceMetrics::default(),
        };
    }
}

impl Component<&EquityMetrics, ()> for PerformanceMetricsComponent {
    fn next(&mut self, equity_metrics: &EquityMetrics) {
        let state = self.sctx.state();

        if let Some(e) = &state.events.on_trade_exit {
            self.data.net_profit = state.net_profit;

            if e.trade.direction == TradeDirection::Long {
                self.data.long_net_profit += e.pnl;
            } else {
                self.data.short_net_profit += e.pnl;
            }

            if e.pnl > 0.0 {
                self.data.gross_profit += e.pnl;
                self.data.winning_trades += 1;
            } else if e.pnl < 0.0 {
                self.data.gross_loss += e.pnl.abs();
                self.data.losing_trades += 1;
            }

            self.data.closed_trades += 1;

            self.data.profitable =
                percent_profitable(self.data.winning_trades, self.data.closed_trades)
                    .unwrap_or(0.0);
            self.data.profit_factor =
                profit_factor(self.data.gross_profit, self.data.gross_loss).unwrap_or(0.0);

            self.data.avg_trade =
                avg_trade(self.data.net_profit, self.data.closed_trades).unwrap_or(0.0);
            self.data.avg_winning_trade =
                avg_winning_trade(self.data.gross_profit, self.data.winning_trades).unwrap_or(0.0);
            self.data.avg_losing_trade =
                avg_losing_trade(self.data.gross_loss, self.data.losing_trades).unwrap_or(0.0);
            self.data.avg_winning_losing_trade_ratio =
                avg_win_loss_ratio(self.data.avg_winning_trade, self.data.avg_losing_trade)
                    .unwrap_or(0.0);
        }

        self.data.open_profit = state.open_profit;

        self.data.max_drawdown = f64::max(
            equity_metrics.net_equity_max - equity_metrics.bar_equity_min,
            self.data.max_drawdown,
        );

        self.data.max_run_up = f64::max(
            equity_metrics.bar_equity_max - equity_metrics.net_equity_min,
            self.data.max_run_up,
        )
    }
}
