use plotters::{
    prelude::{BitMapBackend, ChartBuilder, IntoDrawingArea},
    style::IntoFont,
};
use prettytable::{color, row, Attr, Cell, Row, Table};
use textplots::{Chart, Plot, Shape};

use crate::{
    components::component::Component,
    statistics::stdev_component::StdevComponent,
    strategy::{
        metrics::common::max_run_up_percent, strategy_context::StrategyContext,
        trade::TradeDirection,
    },
    utils::string::with_suffix,
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
};

#[derive(Clone, Copy, Debug)]
pub struct TradingViewMetricsComponentConfig {
    pub risk_free_rate: f64,
}

impl Default for TradingViewMetricsComponentConfig {
    fn default() -> Self {
        Self {
            // Default RFR on TradingView is `2%`.
            risk_free_rate: 0.02,
        }
    }
}

#[derive(Clone, Debug)]
pub struct TradingViewMetrics {
    pub net_profit: f64,
    pub net_profit_percent: f64,
    pub gross_profit: f64,
    pub gross_profit_percent: f64,
    pub gross_loss: f64,
    pub gross_loss_percent: f64,
    pub max_run_up: f64,
    pub max_run_up_percent: f64,
    pub max_drawdown: f64,
    pub max_drawdown_percent: f64,
    pub sharpe_ratio: f64,
    pub sortino_ratio: f64,
    pub profit_factor: f64,
    pub open_pl: f64,
    pub total_closed_trades: usize,
    pub number_winning_trades: usize,
    pub number_losing_trades: usize,
    pub percent_profitable: f64,
    pub avg_trade: f64,
    pub avg_winning_trade: f64,
    pub avg_losing_trade: f64,
    pub ratio_avg_win_avg_loss: f64,
    pub net_equity_history: Vec<f64>,
    pub max_drawdown_history: Vec<f64>,
    pub equity_history: Vec<f64>,
}

impl TradingViewMetrics {
    pub fn default(initial_capital: f64) -> Self {
        return Self {
            net_profit: 0.0,
            net_profit_percent: 0.0,
            gross_profit: 0.0,
            gross_profit_percent: 0.0,
            gross_loss: 0.0,
            gross_loss_percent: 0.0,
            max_run_up: 0.0,
            max_run_up_percent: 0.0,
            max_drawdown: 0.0,
            max_drawdown_percent: 0.0,
            sharpe_ratio: 0.0,
            sortino_ratio: 0.0,
            profit_factor: 0.0,
            open_pl: 0.0,
            total_closed_trades: 0,
            number_winning_trades: 0,
            number_losing_trades: 0,
            percent_profitable: 0.0,
            avg_trade: 0.0,
            avg_winning_trade: 0.0,
            avg_losing_trade: 0.0,
            ratio_avg_win_avg_loss: 0.0,
            net_equity_history: vec![initial_capital],
            max_drawdown_history: vec![0.0],
            equity_history: vec![initial_capital],
        };
    }

    pub fn print_overview(&self) {
        let f_price = with_suffix(" USD");
        let f_percent = with_suffix("%");
        let f = |price: f64, percent: f64| format!("{} {}", f_price(price), f_percent(percent));
        let f_raw = |value: f64| format!("{:0.2}", value);

        let mut table = Table::new();

        let value_cell = |text: &str, theme: i32| {
            let mut cell = Cell::new(text)
                .with_style(Attr::ForegroundColor(color::BRIGHT_WHITE))
                .with_style(Attr::Bold);

            if theme == 1 {
                cell = cell.with_style(Attr::ForegroundColor(color::BRIGHT_GREEN));
            } else if theme == -1 {
                cell = cell.with_style(Attr::ForegroundColor(color::BRIGHT_RED));
            }

            return cell;
        };

        table.add_row(row![
            "Net Profit",
            "Total Closed Trades",
            "Percent Profitable",
            "Profit Factor",
            "Max Drawdown",
            "Avg Trade"
        ]);

        table.add_row(Row::new(vec![
            value_cell(
                &f(self.net_profit, self.net_profit_percent * 100.0).to_string(),
                match self.net_profit {
                    x if x > 0.0 => 1,
                    x if x < 0.0 => -1,
                    _ => 0,
                },
            ),
            value_cell(&self.total_closed_trades.to_string(), 0),
            value_cell(
                &f_percent(self.percent_profitable * 100.0).to_string(),
                match self.percent_profitable {
                    x if x > 0.5 => 1,
                    x if x < 0.5 => -1,
                    _ => 0,
                },
            ),
            value_cell(
                &format!("{:0.3}", self.profit_factor),
                match self.profit_factor {
                    x if x > 1.0 => 1,
                    x if x < 1.0 => -1,
                    _ => 0,
                },
            ),
            value_cell(
                &f(self.max_drawdown, self.max_drawdown_percent * 100.0).to_string(),
                match self.max_drawdown {
                    x if x < 0.2 => 1,
                    x if x > 0.2 => -1,
                    _ => 0,
                },
            ),
            value_cell(&f_price(self.avg_trade).to_string(), 0),
        ]));

        table.printstd();
    }

    pub fn print_summary(&self) {
        let f_price = with_suffix(" USD");
        let f_percent = with_suffix("%");
        let f = |price: f64, percent: f64| format!("{}\n{}", f_price(price), f_percent(percent));
        let f_raw = |value: f64| format!("{:0.2}", value);

        let mut table = Table::new();

        table.add_row(row!["Title", "All"]);
        table.add_row(row![
            "Net Profit",
            f(self.net_profit, self.net_profit_percent * 100.0)
        ]);
        table.add_row(row![
            "Gross Profit",
            f(self.gross_profit, self.gross_profit_percent * 100.0)
        ]);
        table.add_row(row![
            "Gross Loss",
            f(self.gross_loss, self.gross_loss_percent * 100.0)
        ]);
        table.add_row(row![
            "Max Run Up",
            f(self.max_run_up, self.max_run_up_percent * 100.0)
        ]);
        table.add_row(row![
            "Max Drawdown",
            f(self.max_drawdown, self.max_drawdown_percent * 100.0)
        ]);
        table.add_row(row!["Sharpe Ratio", format!("{:0.3}", self.sharpe_ratio)]);
        table.add_row(row!["Sortino Ratio", format!("{:0.3}", self.sortino_ratio)]);
        table.add_row(row!["Profit Factor", format!("{:0.3}", self.profit_factor)]);
        table.add_row(row!["Open P/L", f_price(self.open_pl)]);
        table.add_row(row!["Total Closed Trades", self.total_closed_trades]);
        table.add_row(row!["Number Winning Trades", self.number_winning_trades]);
        table.add_row(row!["Number Losing Trades", self.number_losing_trades]);
        table.add_row(row![
            "Percent Profitable",
            f_percent(self.percent_profitable * 100.0)
        ]);
        table.add_row(row!["Avg Trade", f_price(self.avg_trade)]);
        table.add_row(row!["Avg Winning Trade", f_price(self.avg_winning_trade)]);
        table.add_row(row!["Avg Losing Trade", f_price(self.avg_losing_trade)]);
        table.add_row(row![
            "Ratio Avg Win / Avg Loss",
            f_raw(self.ratio_avg_win_avg_loss)
        ]);

        table.printstd();
    }

    fn text_plot(&self, items: &Vec<f64>, (width, height): (u32, u32)) {
        let data: Vec<(f32, f32)> = items
            .iter()
            .enumerate()
            .map(|(i, &value)| (i as f32 + 1.0, value as f32))
            .collect();

        Chart::new(width, height, 1.0, items.len() as f32)
            .lineplot(&Shape::Lines(&data))
            .nice();
    }

    pub fn plot_net_equity(&self, plot_size: (u32, u32)) {
        self.text_plot(&self.net_equity_history, plot_size);
    }

    pub fn plot_max_drawdown(&self, plot_size: (u32, u32)) {
        self.text_plot(&self.max_drawdown_history, plot_size);
    }

    pub fn plot_equity(&self, plot_size: (u32, u32)) {
        self.text_plot(&self.equity_history, plot_size);
    }
}

/// Includes metrics from TradingView "Performance Summary" tab.
///
/// Some metrics are not exact: `Sharpe Ratio, Sortino Ratio`.
///
/// `TradingViewMetricsComponent` is intented to be used while developing a strategy, as it is not optimized for performance.
pub struct TradingViewMetricsComponent {
    pub sctx: StrategyContext,
    pub data: TradingViewMetrics,
    pub config: TradingViewMetricsComponentConfig,
    returns: ReturnsComponent,
    neg_returns_stdev: StdevComponent,
}

impl TradingViewMetricsComponent {
    pub fn new(sctx: StrategyContext, config: TradingViewMetricsComponentConfig) -> Self {
        return Self {
            sctx: sctx.clone(),
            data: TradingViewMetrics::default(sctx.initial_capital),
            config,
            returns: ReturnsComponent::new(sctx.ctx.clone(), sctx.state().config.initial_capital),
            neg_returns_stdev: StdevComponent::new(sctx.ctx.clone()),
        };
    }
}

impl Component<(&EquityMetrics, &PerformanceMetrics), ()> for TradingViewMetricsComponent {
    fn next(&mut self, (equity_metrics, perf_metrics): (&EquityMetrics, &PerformanceMetrics)) {
        let initial_capital = self.sctx.initial_capital;

        let state = self.sctx.state();

        if let Some(e) = &state.events.on_trade_exit {
            self.data.net_equity_history.push(equity_metrics.net_equity);
            self.data
                .max_drawdown_history
                .push(perf_metrics.max_drawdown);
        }

        self.data.equity_history.push(equity_metrics.equity);

        self.returns.next(equity_metrics.net_equity);
        let returns = &self.returns.data;

        let neg_returns_stdev = self
            .neg_returns_stdev
            .next(f64::min(0.0, returns.delta).abs());

        self.data.net_profit = perf_metrics.net_profit;
        self.data.net_profit_percent = net_profit_percent(perf_metrics.net_profit, initial_capital);

        self.data.gross_profit = perf_metrics.gross_profit;
        self.data.gross_profit_percent =
            gross_profit_percent(perf_metrics.gross_profit, initial_capital);

        self.data.gross_loss = perf_metrics.gross_loss;
        self.data.gross_loss_percent =
            gross_profit_percent(perf_metrics.gross_loss, initial_capital);

        self.data.max_drawdown = perf_metrics.max_drawdown;
        self.data.max_drawdown_percent =
            max_drawdown_percent(perf_metrics.max_drawdown, equity_metrics.net_equity_max);

        self.data.max_run_up = perf_metrics.max_run_up;
        self.data.max_run_up_percent =
            max_run_up_percent(perf_metrics.max_run_up, equity_metrics.bar_equity_max);

        self.data.profit_factor = perf_metrics.profit_factor;
        self.data.open_pl = perf_metrics.open_profit;
        self.data.total_closed_trades = perf_metrics.closed_trades;
        self.data.number_winning_trades = perf_metrics.winning_trades;
        self.data.number_losing_trades = perf_metrics.losing_trades;
        self.data.percent_profitable =
            percent_profitable(perf_metrics.winning_trades, perf_metrics.closed_trades)
                .unwrap_or(0.0);
        self.data.avg_trade =
            avg_trade(perf_metrics.net_profit, perf_metrics.closed_trades).unwrap_or(0.0);
        self.data.avg_winning_trade =
            avg_winning_trade(self.data.gross_profit, self.data.number_winning_trades)
                .unwrap_or(0.0);
        self.data.avg_losing_trade =
            avg_losing_trade(self.data.gross_loss, self.data.number_losing_trades).unwrap_or(0.0);

        self.data.ratio_avg_win_avg_loss =
            avg_win_loss_ratio(self.data.avg_winning_trade, self.data.avg_losing_trade)
                .unwrap_or(0.0);

        self.data.sharpe_ratio =
            sharpe_ratio(returns.mean, returns.stdev, self.config.risk_free_rate);
        self.data.sortino_ratio =
            sortino_ratio(returns.mean, neg_returns_stdev, self.config.risk_free_rate);
    }
}
