// use std::rc::Rc;

// use crate::base::components::component_context::ComponentContext;

// use super::{
//     runnable_strategy::RunnableStrategy, strategy_execution_context::StrategyExecutionContextConfig,
// };

// pub struct StrategyRunner {
//     ctx: ComponentContext,
//     strategy_ctx: StrategyExecutionContextConfig,
//     strategy: Box<dyn RunnableStrategy + 'static>,
// }

// impl StrategyRunner {
//     pub fn new(
//         ctx: ComponentContext,
//         strategy_ctx: StrategyExecutionContextConfig,
//         strategy: Box<dyn RunnableStrategy + 'static>,
//     ) -> Self {
//         return Self {
//             ctx: ctx.clone(),
//             strategy_ctx,
//         };
//     }

//     pub fn run() {}

//     // fn next(&self) -> ;
// }
// pub struct StrategyRunnerConfig {}

use std::borrow::BorrowMut;

use colored::Colorize;
use polars::export::rayon::vec;

use super::{
    metrics::{
        omega_ratio_metric::{OmegaRatioMetric, OmegaRatioMetricConfig},
        sharpe_ratio_metric::{SharpeRatioMetric, SharpeRatioMetricConfig},
    },
    strategy_context::StrategyContext,
    trade::{Trade, TradeDirection},
};
use crate::{
    base::{
        components::{
            common::{
                mean_component::MeanComponent,
                welfords_stdev_component::WelfordsStandardDeviationComponent,
            },
            component_context::ComponentContext,
            component_default::ComponentDefault,
        },
        strategy::{metrics::profit::compute_profit_factor, trade::compute_return},
    },
    content::{
        relative_strength_index_indicator::RelativeStrengthIndexIndicator,
        relative_strength_index_strategy::RelativeStrengthIndexStrategy,
    },
};
use pyo3::prelude::*;

pub struct StrategyRunnerConfig {
    pub print: bool,
    pub start_tick: Option<usize>,
    pub end_tick: Option<usize>,
    pub metrics: StrategyRunnerMetricsConfig,
}

impl ComponentDefault for StrategyRunnerConfig {
    fn default(ctx: ComponentContext) -> Self {
        return Self {
            print: false,
            start_tick: None,
            end_tick: None,
            metrics: StrategyRunnerMetricsConfig::default(ctx.clone()),
        };
    }
}

pub struct StrategyRunnerMetricsConfig {
    pub omega_ratio: Option<OmegaRatioMetric>,
    pub sharpe_ratio: Option<SharpeRatioMetric>,
    pub track: bool,
}

impl ComponentDefault for StrategyRunnerMetricsConfig {
    fn default(ctx: ComponentContext) -> Self {
        return Self {
            omega_ratio: Some(OmegaRatioMetric::new(
                ctx.clone(),
                OmegaRatioMetricConfig::default(ctx.clone()),
            )),
            sharpe_ratio: Some(SharpeRatioMetric::new(
                ctx.clone(),
                SharpeRatioMetricConfig::default(ctx.clone()),
            )),
            track: false,
        };
    }
}

#[derive(Debug, Clone)]
#[pyclass(name = "StrategyRunnerResult")]
pub struct StrategyRunnerResult {
    #[pyo3(get)]
    pub metrics: StrategyRunnerMetrics,
    #[pyo3(get)]
    pub metrics_history: Vec<StrategyRunnerMetrics>,
    #[pyo3(get)]
    pub trades: Vec<Trade>,
}

#[derive(Debug, Clone, Copy)]
#[pyclass(name = "StrategyRunnerMetrics")]
pub struct StrategyRunnerMetrics {
    #[pyo3(get)]
    pub tick: usize,
    #[pyo3(get)]
    pub time: u128,
    #[pyo3(get)]
    pub equity: f64,
    #[pyo3(get)]
    pub open_profit: f64,
    #[pyo3(get)]
    pub net_profit: f64,
    #[pyo3(get)]
    pub gross_profit: f64,
    #[pyo3(get)]
    pub gross_loss: f64,
    #[pyo3(get)]
    pub profit_factor: f64,
    #[pyo3(get)]
    pub returns: f64,
    #[pyo3(get)]
    pub total_closed_trades: usize,
    #[pyo3(get)]
    pub number_of_winning_trades: usize,
    #[pyo3(get)]
    pub number_of_losing_trades: usize,
    #[pyo3(get)]
    pub percent_profitable: f64,
    #[pyo3(get)]
    pub sharpe_ratio: Option<f64>,
    #[pyo3(get)]
    pub omega_ratio: Option<f64>,
}

pub struct StrategyRunner {
    pub config: StrategyRunnerConfig,
    pub ctx: ComponentContext,
    pub strategy_ctx: StrategyContext,
    already_run: bool,
    strategy_start_tick: usize,
    strategy_end_tick: usize,
    returns_stdev: WelfordsStandardDeviationComponent,
    returns_mean: MeanComponent,
}

impl StrategyRunner {
    pub fn new(
        ctx: ComponentContext,
        strategy_ctx: StrategyContext,
        config: StrategyRunnerConfig,
    ) -> Self {
        let strategy_start_tick = config.start_tick.unwrap_or(ctx.get().start_tick());
        let strategy_end_tick = config.end_tick.unwrap_or(ctx.get().end_tick());

        return Self {
            ctx: ctx.clone(),
            strategy_ctx,
            already_run: false,
            strategy_start_tick,
            strategy_end_tick,
            returns_stdev: WelfordsStandardDeviationComponent::new(ctx.clone()),
            returns_mean: MeanComponent::new(ctx.clone()),
            config,
        };
    }

    fn print(ctx: ComponentContext, current_trade: Option<&Trade>, res: &StrategyRunnerResult) {
        let ctx = ctx.get();
        let tick = ctx.current_tick;
        let open = ctx.open().unwrap_or(0.0);
        let close = ctx.close().unwrap_or(0.0);
        let date = ctx.datetime().unwrap().format("%d-%m-%Y %H:%M");

        println!(
            "\n{} {} {}: OPEN: {}  CLOSE: {}",
            if let Some(current_trade) = current_trade {
                current_trade.get_triangle_colored_string(tick)
            } else {
                "—".to_string().black().bold()
            },
            date.to_string().bright_black(),
            format!("[{}]", tick).bright_black(),
            open.to_string().bright_blue(),
            close.to_string().bright_blue(),
        );

        // if let Some(equity) = res.metrics.equity {
        //     let metrics_indentation = " ".repeat(19);
        //     let metrics_separator = "-".repeat(64);
        //     let metrics_separator = format!("{metrics_indentation}{metrics_separator}").black();
        //     println!("{}", metrics_separator);

        //     println!(
        //         "{metrics_indentation}Capital: {}",
        //         format!("{:0.8}", equity.capital).bright_magenta(),
        //     );
        //     println!(
        //         "{metrics_indentation}PnL: {}",
        //         format!("{:0.2}", equity.trade_pnl).bright_magenta(),
        //     );
        //     println!(
        //         "{metrics_indentation}Returns: {}",
        //         format!("{:0.2}", equity.returns).bright_magenta(),
        //     );

        //     if let Some(current_trade) = current_trade {
        //         println!(
        //             "{metrics_indentation}Trade Direction: {}",
        //             format!("{:?}", current_trade.direction).bright_magenta(),
        //         );
        //         if let Some(entry_price) = current_trade.entry_price {
        //             println!(
        //                 "{metrics_indentation}Fill Price: {}",
        //                 format!("{}", entry_price).bright_magenta(),
        //             );
        //         }
        //         if let Some(entry_tick) = current_trade.entry_tick {
        //             println!(
        //                 "{metrics_indentation}Fill Tick: {}",
        //                 format!("{:?}", entry_tick).bright_magenta(),
        //             );
        //         }
        //         if let Some(fill_size) = equity.trade_fill_size {
        //             println!(
        //                 "{metrics_indentation}Fill Size: {}",
        //                 format!("{:?}", fill_size).bright_magenta(),
        //             )
        //         }
        //     }

        //     if let Some(sharpe_ratio) = res.metrics.sharpe_ratio {
        //         println!(
        //             "{metrics_indentation}Sharpe Ratio: {}",
        //             format!("{:0.2}", sharpe_ratio).bright_magenta(),
        //         );
        //     }

        //     if let Some(omega_ratio) = res.metrics.omega_ratio {
        //         println!(
        //             "{metrics_indentation}Omega Ratio: {}",
        //             format!("{:0.2}", omega_ratio).bright_magenta(),
        //         );
        //     }
        //     println!("{}", metrics_separator);
        // }
    }

    pub fn run<F: FnMut() -> Option<TradeDirection>>(&mut self, mut cb: F) -> StrategyRunnerResult {
        assert!(!self.already_run, "StrategyRunner can only be run once");
        self.already_run = true;

        let start = self.ctx.get().start_tick();
        let end = self.strategy_end_tick;

        let mut res = StrategyRunnerResult {
            metrics: StrategyRunnerMetrics {
                time: 0,
                tick: 0,
                equity: 0.0,
                open_profit: 0.0,
                net_profit: 0.0,
                returns: 0.0,
                sharpe_ratio: None,
                omega_ratio: None,
                gross_loss: 0.0,
                total_closed_trades: 0,
                gross_profit: 0.0,
                number_of_losing_trades: 0,
                number_of_winning_trades: 0,
                percent_profitable: 0.0,
                profit_factor: 0.0,
            },
            metrics_history: vec![],
            trades: vec![],
        };

        let mut prev_equity = self.strategy_ctx.config.initial_capital;

        for tick in start..end + 1 {
            self.ctx.get_mutable().current_tick = tick;

            let trade_direction = cb();

            if tick >= self.strategy_start_tick {
                res.metrics.tick = tick;
                res.metrics.time = self.ctx.get().time().unwrap().as_millis();

                self.strategy_ctx.next(trade_direction);

                if self.strategy_ctx.on_close_trade {
                    let metrics = &self.strategy_ctx.metrics;

                    res.metrics.equity = metrics.equity;
                    res.metrics.open_profit = metrics.open_profit;
                    res.metrics.net_profit = metrics.net_profit;
                    res.metrics.gross_profit = metrics.gross_profit;
                    res.metrics.gross_loss = metrics.gross_loss;
                    // res.metrics.percent_profitable = metrics.percent_profitable;
                    // res.metrics.number_of_winning_trades = metrics.number_of_winning_trades;
                    // res.metrics.number_of_losing_trades = metrics.number_of_losing_trades;
                    res.metrics.profit_factor =
                        compute_profit_factor(metrics.gross_profit, metrics.gross_loss);
                    // res.metrics.total_closed_trades = metrics.total_closed_trades;

                    res.metrics.returns = compute_return(
                        metrics.net_profit + self.strategy_ctx.config.initial_capital,
                        prev_equity,
                    );
                    let returns_mean = self.returns_mean.next(res.metrics.returns);
                    let returns_stdev = self.returns_stdev.next(res.metrics.returns);

                    prev_equity = metrics.net_profit + self.strategy_ctx.config.initial_capital;

                    if let Some(sharpe_ratio_metric) = &mut self.config.metrics.sharpe_ratio {
                        res.metrics.sharpe_ratio = Some(
                            sharpe_ratio_metric.next(returns_mean, returns_stdev)
                                * f64::sqrt(365.0),
                        );
                    }

                    if let Some(omega_ratio_metric) = &mut self.config.metrics.omega_ratio {
                        res.metrics.omega_ratio =
                            Some(omega_ratio_metric.next(res.metrics.returns) * f64::sqrt(365.0));
                    }
                    // println!("[{}]: {:?}", tick, res.metrics);

                    if self.config.metrics.track {
                        res.metrics_history.push(res.metrics.clone());
                    }
                } else {
                    // self.returns_mean.next(0.0);
                    // self.returns_stdev.next(0.0);
                }

                // res.metrics.equity = metrics.equity;
                // res.metrics.open_profit = metrics.open_profit;
                // res.metrics.net_profit = metrics.net_profit;
                // res.metrics.returns = metrics.returns;

                // if let Some(sharpe_ratio_metric) = &mut self.config.metrics.sharpe_ratio {
                //     res.metrics.sharpe_ratio = Some(
                //         sharpe_ratio_metric.next(metrics.returns_mean, metrics.returns_stdev)
                //             * f64::sqrt(365.0),
                //     );
                // }

                // if let Some(omega_ratio_metric) = &mut self.config.metrics.omega_ratio {
                //     res.metrics.omega_ratio = Some(omega_ratio_metric.next(metrics.returns));
                // }

                // if self.config.print {
                //     Self::print(self.ctx.clone(), trade, &res);
                // }
            }
        }

        if self.config.metrics.track {
            res.trades.extend(self.strategy_ctx.trades.clone());
        }

        return res;
    }
}
