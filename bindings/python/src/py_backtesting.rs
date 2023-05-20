use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use nersent_pace::{
    core::{context::Context, data_provider::AnyDataProvider, incremental::Incremental},
    pinescript::pinescript_exporter::{PineScriptExportStrategyConfig, PineScriptExporter},
    strategy::{
        metrics::{
            cobra_metrics::{CobraMetrics, CobraMetricsConfig},
            equity_metrics::EquityMetrics,
            tradingview_metrics::{TradingViewMetrics, TradingViewMetricsConfig},
        },
        strategy::{Strategy, StrategyConfig},
        trade::{trade_direction_to_f64, StrategySignal},
    },
    utils::float::OptionFloatUtils,
};
use pyo3::{prelude::*, types::PyDict};

use crate::{
    py_data_provider::PyDataProvider, py_strategy::PyStrategySignal, pyo3_utils::PyAnyCast,
};

struct PyBacktestRunnerConfig {
    pub strategy_config: StrategyConfig,
    pub tradingview_metrics_config: TradingViewMetricsConfig,
    pub cobra_metrics_config: CobraMetricsConfig,
    pub pct_multiplier: f64,
}

impl PyBacktestRunnerConfig {
    pub fn from_dict(data: &PyDict) -> Self {
        return Self {
            strategy_config: StrategyConfig {
                on_bar_close: data.get_item("on_bar_close").unwrap().to_bool(),
                initial_capital: data.get_item("initial_capital").unwrap().to_f64(),
                buy_with_equity: data.get_item("buy_with_equity").unwrap().to_bool(),
            },
            tradingview_metrics_config: TradingViewMetricsConfig {
                risk_free_rate: data.get_item("risk_free_rate").unwrap().to_f64(),
            },
            cobra_metrics_config: CobraMetricsConfig {
                estimated: false,
                returns_start_year: None,
            },
            pct_multiplier: 100.0,
        };
    }
}

#[derive(Clone)]
#[pyclass(name = "BacktestBarInfo")]
pub struct PyBacktestBarInfo {
    #[pyo3(get)]
    pub tick: f64,
    #[pyo3(get)]
    pub time: f64,
    #[pyo3(get)]
    pub equity: f64,
    #[pyo3(get)]
    pub net_equity: f64,
    #[pyo3(get)]
    pub position_size: f64,
    #[pyo3(get)]
    pub direction: f64,
    #[pyo3(get)]
    pub open_profit: f64,
    #[pyo3(get)]
    pub returns: f64,
    #[pyo3(get)]
    pub logs: String,
    #[pyo3(get)]
    pub omega_ratio: f64,
    #[pyo3(get)]
    pub sharpe_ratio: f64,
    #[pyo3(get)]
    pub sortino_ratio: f64,
    #[pyo3(get)]
    pub profitable: f64,
    #[pyo3(get)]
    pub max_drawdown: f64,
    #[pyo3(get)]
    pub max_drawdown_percent: f64,
    #[pyo3(get)]
    pub max_run_up: f64,
    #[pyo3(get)]
    pub max_run_up_percent: f64,
    #[pyo3(get)]
    pub net_profit: f64,
    #[pyo3(get)]
    pub net_profit_percent: f64,
    #[pyo3(get)]
    pub gross_profit: f64,
    #[pyo3(get)]
    pub gross_profit_percent: f64,
    #[pyo3(get)]
    pub gross_loss: f64,
    #[pyo3(get)]
    pub gross_loss_percent: f64,
    #[pyo3(get)]
    pub closed_trades: usize,
    #[pyo3(get)]
    pub winning_trades: usize,
    #[pyo3(get)]
    pub losing_trades: usize,
    #[pyo3(get)]
    pub profit_factor: f64,
    #[pyo3(get)]
    pub equity_curve_max_drawdown: f64,
    #[pyo3(get)]
    pub intra_trade_max_drawdown: f64,
    #[pyo3(get)]
    pub net_profit_l_s_ratio: f64,
}

struct PyBacktestRunner {
    pub ctx: Context,
    pub config: PyBacktestRunnerConfig,
    pub strategy: Strategy,
    pub tradingview_metrics: TradingViewMetrics,
    pub cobra_metrics: CobraMetrics,
    pub equity_metrics: EquityMetrics,
    pub bars: Vec<PyBacktestBarInfo>,
}

impl PyBacktestRunner {
    pub fn new(data_provider: AnyDataProvider, config: PyBacktestRunnerConfig) -> Self {
        let ctx = Context::new(data_provider);
        let strategy = Strategy::new(ctx.clone(), config.strategy_config);
        let tradingview_metrics =
            TradingViewMetrics::new(ctx.clone(), config.tradingview_metrics_config, &strategy);
        let cobra_metrics = CobraMetrics::new(ctx.clone(), config.cobra_metrics_config, &strategy);
        let equity_metrics = EquityMetrics::new(ctx.clone(), &strategy);

        return Self {
            ctx,
            config,
            strategy,
            tradingview_metrics,
            cobra_metrics,
            equity_metrics,
            bars: vec![],
        };
    }

    pub fn run(&mut self, signals: Vec<PyStrategySignal>) -> PyBacktestResult {
        let run_start_time = Instant::now();

        for i in self.ctx.first_bar_index..=self.ctx.last_bar_index {
            self.ctx.bar.index.set(i);

            let bar = &self.ctx.bar;
            let time = bar.time().unwrap();
            let time_s = time.as_secs() as f64;

            let mut signal = signals[i].get();

            self.strategy.next_bar();

            if self.strategy.config.on_bar_close {
                self.strategy.next(signal);
            }

            self.equity_metrics.next(&self.strategy);
            self.tradingview_metrics.next(&self.strategy);
            self.cobra_metrics.next(&self.strategy);

            let mut logs: Vec<String> = vec![];

            if let Some(e) = &self.strategy.events.on_trade_exit {
                logs.push(format!(
                    "[nersent_pace::event::on_trade_exit: {:?}",
                    e.trade
                ));
            }

            if let Some(e) = &self.strategy.events.on_trade_entry {
                logs.push(format!(
                    "[nersent_pace::event::on_trade_enty]: {:?}",
                    e.trade
                ));
            }

            let logs: String = logs.join("\n\n");

            let bar_info = PyBacktestBarInfo {
                tick: bar.index() as f64,
                time: time_s,
                equity: self.strategy.metrics.equity,
                returns: 0.0,
                net_equity: self.equity_metrics.data.net_equity,
                open_profit: self.strategy.metrics.open_profit,
                position_size: self.strategy.metrics.position_size,
                direction: trade_direction_to_f64(self.strategy.current_dir),
                omega_ratio: self.cobra_metrics.data.omega,
                sharpe_ratio: self.cobra_metrics.data.sharpe,
                sortino_ratio: self.cobra_metrics.data.sortino,
                profitable: self.tradingview_metrics.data.percent_profitable
                    * self.config.pct_multiplier,
                max_drawdown: self.tradingview_metrics.data.max_drawdown,
                max_drawdown_percent: self.tradingview_metrics.data.max_drawdown_percent
                    * self.config.pct_multiplier,
                max_run_up: self.tradingview_metrics.data.max_run_up,
                max_run_up_percent: self.tradingview_metrics.data.max_run_up_percent
                    * self.config.pct_multiplier,
                net_profit: self.tradingview_metrics.data.net_profit,
                net_profit_percent: self.tradingview_metrics.data.net_profit_percent
                    * self.config.pct_multiplier,
                gross_profit: self.tradingview_metrics.data.gross_profit,
                gross_profit_percent: self.tradingview_metrics.data.gross_profit_percent
                    * self.config.pct_multiplier,
                gross_loss: self.tradingview_metrics.data.gross_loss,
                gross_loss_percent: self.tradingview_metrics.data.gross_loss_percent
                    * self.config.pct_multiplier,
                closed_trades: self.tradingview_metrics.data.total_closed_trades,
                winning_trades: self.tradingview_metrics.data.number_winning_trades,
                losing_trades: self.tradingview_metrics.data.number_losing_trades,
                profit_factor: self.tradingview_metrics.data.profit_factor,
                logs,
                equity_curve_max_drawdown: self.cobra_metrics.data.equity_curve_max_dd
                    * self.config.pct_multiplier,
                intra_trade_max_drawdown: self.cobra_metrics.data.intra_trade_max_dd
                    * self.config.pct_multiplier,
                net_profit_l_s_ratio: self.cobra_metrics.data.net_profit_l_s_ratio,
            };
            self.bars.push(bar_info);

            if !self.strategy.config.on_bar_close {
                self.strategy.next(signal);
            }
        }

        let run_end_time = Instant::now();
        let run_time = run_end_time.duration_since(run_start_time).as_secs_f64();

        let ps_exporter = PineScriptExporter::new();
        let pinescript = ps_exporter.strategy(
            &self.strategy,
            PineScriptExportStrategyConfig {
                include_cobra_metrics: true,
                risk_free_rate: self.config.tradingview_metrics_config.risk_free_rate,
                title: "Pace Strategy".to_string(),
                ..PineScriptExportStrategyConfig::default()
            },
        );

        return PyBacktestResult {
            run_time,
            bars: self.bars.clone(),
            pinescript,
        };
    }
}

#[derive(Clone)]
#[pyclass(name = "BacktestResult")]
pub struct PyBacktestResult {
    #[pyo3(get)]
    pub run_time: f64,
    #[pyo3(get)]
    pub bars: Vec<PyBacktestBarInfo>,
    #[pyo3(get)]
    pub pinescript: String,
}

#[pyfunction]
pub fn run_backtest(
    py: Python<'_>,
    data_provider: &PyDataProvider,
    config: &PyDict,
    signals: Vec<PyStrategySignal>,
) -> PyBacktestResult {
    let mut runner = PyBacktestRunner::new(
        data_provider.get(),
        PyBacktestRunnerConfig::from_dict(config),
    );

    return runner.run(signals);
}
