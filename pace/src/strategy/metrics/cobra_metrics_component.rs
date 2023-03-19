use chrono::Datelike;
use prettytable::{color, row, Attr, Cell, Row, Table};

use crate::{
    components::component::Component,
    statistics::{
        mean_component::MeanComponent, stdev_component::StdevComponent,
        welfords_stdev_component::WelfordsStdevComponent,
    },
    strategy::{strategy_context::StrategyContext, trade::TradeDirection},
    ta::sum_component::SumComponent,
    utils::string::with_suffix,
};

use super::{
    common::{
        long_net_profit_ratio, omega_ratio, percent_profitable, profit_factor, returns,
        sharpe_ratio, sortino_ratio,
    },
    equity_metrics_component::EquityMetrics,
    omega_ratio_component::{OmegaRatioComponent, OmegaRatioComponentConfig},
    performance_metrics_component::PerformanceMetrics,
    sharpe_ratio_component::{SharpeRatioComponent, SharpeRatioComponentConfig},
};

#[derive(Debug, Copy, Clone)]
pub struct CobraMetrics {
    pub equity_curve_max_dd: f64,
    pub intra_trade_max_dd: f64,
    pub sortino: f64,
    pub sharpe: f64,
    pub profit_factor: f64,
    pub profitable: f64,
    pub trades: usize,
    pub omega: f64,
    pub net_profit_l_s_ratio: f64,
}

impl CobraMetrics {
    pub fn default() -> Self {
        return Self {
            equity_curve_max_dd: 0.0,
            intra_trade_max_dd: 0.0,
            sortino: 0.0,
            sharpe: 0.0,
            profit_factor: 0.0,
            profitable: 0.0,
            trades: 0,
            omega: 0.0,
            net_profit_l_s_ratio: 0.0,
        };
    }

    pub fn print(&self) {
        let f_percent = with_suffix("%");
        let f_raw = |value: f64| format!("{:0.2}", value);

        let key_cell = |text: &str| {
            let mut cell = Cell::new(text)
                .with_style(Attr::ForegroundColor(color::BRIGHT_YELLOW))
                .with_style(Attr::Bold);
            return cell;
        };

        let value_cell = |text: &str, theme: i32| {
            let mut cell = Cell::new(text)
                .with_style(Attr::ForegroundColor(color::YELLOW))
                .with_style(Attr::Bold);

            if theme == 1 {
                cell = cell.with_style(Attr::ForegroundColor(color::GREEN));
            } else if theme == -1 {
                cell = cell.with_style(Attr::ForegroundColor(color::RED));
            }

            return cell;
        };

        let mut table = Table::new();

        table.add_row(Row::new(vec![
            key_cell("Equity Curve Max DD"),
            value_cell(
                &f_percent(self.equity_curve_max_dd * 100.0),
                match self.equity_curve_max_dd {
                    x if x > -0.25 => 1,
                    x if x < -0.40 => -1,
                    _ => 0,
                },
            ),
        ]));

        table.add_row(Row::new(vec![
            key_cell("Intra-trade Max DD"),
            value_cell(
                &f_percent(self.intra_trade_max_dd * 100.0),
                match self.intra_trade_max_dd {
                    x if x > -0.25 => 1,
                    x if x < -0.40 => -1,
                    _ => 0,
                },
            ),
        ]));

        table.add_row(Row::new(vec![
            key_cell("Sortino"),
            value_cell(
                &f_raw(self.sortino),
                match self.sortino {
                    x if x > 3.0 => 1,
                    x if x < 2.0 => -1,
                    _ => 0,
                },
            ),
        ]));

        table.add_row(Row::new(vec![
            key_cell("Sharpe"),
            value_cell(
                &f_raw(self.sharpe),
                match self.sharpe {
                    x if x > 1.5 => 1,
                    x if x < 1.0 => -1,
                    _ => 0,
                },
            ),
        ]));

        table.add_row(Row::new(vec![
            key_cell("Profit Factor"),
            value_cell(
                &f_raw(self.profit_factor),
                match self.profit_factor {
                    x if x > 4.0 => 1,
                    x if x < 2.0 => -1,
                    _ => 0,
                },
            ),
        ]));

        table.add_row(Row::new(vec![
            key_cell("Profitable %"),
            value_cell(
                &f_percent(self.profitable * 100.0),
                match self.profitable {
                    x if x > 0.50 => 1,
                    x if x < 0.35 => -1,
                    _ => 0,
                },
            ),
        ]));

        table.add_row(Row::new(vec![
            key_cell("Trades"),
            value_cell(
                &f_raw(self.trades as f64),
                match self.trades {
                    x if x >= 20 && x <= 80 => 1,
                    x if x < 15 || x > 80 => -1,
                    _ => 0,
                },
            ),
        ]));

        table.add_row(Row::new(vec![
            key_cell("Omega"),
            value_cell(&f_raw(self.omega), 0),
        ]));

        table.add_row(Row::new(vec![
            key_cell("Net Profit L/S Ratio"),
            value_cell(&f_raw(self.net_profit_l_s_ratio), 0),
        ]));

        table.printstd();
    }
}

#[derive(Debug, Copy, Clone)]
pub struct CobraMetricsComponentConfig {
    pub estimated: bool,
    pub returns_start_year: Option<i32>,
}

impl Default for CobraMetricsComponentConfig {
    fn default() -> Self {
        return Self {
            estimated: false,
            returns_start_year: Some(2018),
        };
    }
}

/// Ported from https://www.tradingview.com/v/MN8HOZ5M/
pub struct CobraMetricsComponent {
    pub sctx: StrategyContext,
    pub config: CobraMetricsComponentConfig,
    pub data: CobraMetrics,
    current_trade_max_drawdown: f64,
    annualized: f64,
    returns_mean: MeanComponent,
    returns_stdev: StdevComponent,
    positive_returns_sum: f64,
    negative_returns_stdev: StdevComponent,
    negative_returns_sum: f64,
    prev_equity: f64,
    risk_free_rate: f64,
}

impl CobraMetricsComponent {
    pub fn new(sctx: StrategyContext, config: CobraMetricsComponentConfig) -> Self {
        let ctx = sctx.ctx.clone();
        let state = sctx.state();
        return Self {
            sctx: sctx.clone(),
            data: CobraMetrics::default(),
            current_trade_max_drawdown: 0.0,
            risk_free_rate: 0.0,
            annualized: f64::sqrt(365.0),
            returns_mean: MeanComponent::new(ctx.clone()),
            returns_stdev: StdevComponent::build(ctx.clone(), config.estimated),
            negative_returns_sum: 0.0,
            negative_returns_stdev: StdevComponent::build(ctx.clone(), config.estimated),
            positive_returns_sum: 0.0,
            prev_equity: state.equity,
            config,
        };
    }
}

impl Component<(&EquityMetrics, &PerformanceMetrics), ()> for CobraMetricsComponent {
    fn next(&mut self, (equity_metrics, perf_metrics): (&EquityMetrics, &PerformanceMetrics)) {
        let state = self.sctx.state();

        if let Some(e) = &state.events.on_trade_exit {
            self.data.profitable = perf_metrics.profitable;
            self.data.profit_factor = perf_metrics.profit_factor;
            self.data.trades = perf_metrics.closed_trades;

            let intra_trade_max_drawdown_percent =
                self.current_trade_max_drawdown / e.trade.entry_price.unwrap();

            self.data.intra_trade_max_dd = f64::max(
                intra_trade_max_drawdown_percent,
                self.data.intra_trade_max_dd,
            );

            self.current_trade_max_drawdown = 0.0;

            self.data.net_profit_l_s_ratio =
                long_net_profit_ratio(perf_metrics.long_net_profit, perf_metrics.short_net_profit)
                    .unwrap_or(0.0);
        }

        self.data.equity_curve_max_dd = f64::min(
            equity_metrics.equity / equity_metrics.equity_max - 1.0,
            self.data.equity_curve_max_dd,
        );

        self.current_trade_max_drawdown = f64::max(
            equity_metrics.net_equity - equity_metrics.bar_equity_min,
            self.current_trade_max_drawdown,
        );

        let equity_returns = returns(equity_metrics.equity, self.prev_equity);
        self.prev_equity = equity_metrics.equity;

        if let Some(returns_start_year) = self.config.returns_start_year {
            let bar_year = self.sctx.ctx.datetime().unwrap().year();
            if bar_year < returns_start_year {
                return;
            }
        }

        let returns_mean = self.returns_mean.next(equity_returns);
        let returns_stdev = self.returns_stdev.next(equity_returns);

        let positive_returns = f64::max(0.0, equity_returns);
        let negative_returns = f64::min(0.0, equity_returns).abs();
        let negative_returns_stdev = self.negative_returns_stdev.next(negative_returns);

        self.positive_returns_sum += positive_returns;
        self.negative_returns_sum += negative_returns;

        self.data.omega = omega_ratio(
            self.positive_returns_sum,
            self.negative_returns_sum,
            self.risk_free_rate,
        ) * self.annualized;

        self.data.sharpe =
            sharpe_ratio(returns_mean, returns_stdev, self.risk_free_rate) * self.annualized;

        self.data.sortino =
            sortino_ratio(returns_mean, negative_returns_stdev, self.risk_free_rate)
                * self.annualized;
    }
}
