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

use crate::{
    base::{
        components::{component_context::ComponentContext, component_default::ComponentDefault},
        strategy::metrics::equity_metric::Equity,
    },
    content::{
        relative_strength_index_indicator::RelativeStrengthIndexIndicator,
        relative_strength_index_strategy::RelativeStrengthIndexStrategy,
    },
};

use super::{
    metrics::{
        equity_metric::{EquityMetric, EquityMetricConfig},
        metric::{MetricComponentResultUnion, MetricComponentUnion},
        omega_ratio_metric::{OmegaRatioMetric, OmegaRatioMetricConfig},
        sharpe_ratio_metric::{SharpeRatioMetric, SharpeRatioMetricConfig},
        total_closed_trades_metric::TotalClosedTradesMetric,
    },
    strategy_execution_context::StrategyExecutionContext,
    trade::{Trade, TradeDirection},
};

pub struct StrategyRunnerConfig {
    pub print: bool,
    pub start_tick: Option<usize>,
    pub end_tick: Option<usize>,
}

impl ComponentDefault for StrategyRunnerConfig {
    fn default(ctx: ComponentContext) -> Self {
        return Self {
            print: false,
            start_tick: None,
            end_tick: None,
        };
    }
}

pub struct StrategyRunnerResult {
    pub tick: usize,
    pub metrics: Vec<MetricComponentResultUnion>,
}

pub struct StrategyRunner {
    pub ctx: ComponentContext,
    pub strategy_ctx: StrategyExecutionContext,
    already_run: bool,
    strategy_start_tick: usize,
    strategy_end_tick: usize,
    config: StrategyRunnerConfig,
    equity: EquityMetric,
    metrics: Vec<MetricComponentUnion>,
}

impl StrategyRunner {
    pub fn new(
        ctx: ComponentContext,
        strategy_ctx: StrategyExecutionContext,
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
            config,
            equity: EquityMetric::new(
                ctx.clone(),
                EquityMetricConfig {
                    initial_capital: 10000.0,
                },
            ),
            metrics: vec![
                MetricComponentUnion::OmegaRatio(OmegaRatioMetric::new(
                    ctx.clone(),
                    OmegaRatioMetricConfig {
                        risk_free_rate: 0.0,
                    },
                )),
                MetricComponentUnion::SharpeRatio(SharpeRatioMetric::new(
                    ctx.clone(),
                    SharpeRatioMetricConfig {
                        risk_free_rate: 0.0,
                    },
                )),
                MetricComponentUnion::TotalClosedTrades(TotalClosedTradesMetric::new(ctx.clone())),
            ],
        };
    }

    fn print(ctx: ComponentContext, current_trade: Option<&Trade>, res: StrategyRunnerResult) {
        let ctx = ctx.get();
        let tick = ctx.current_tick;
        let open = ctx.open().unwrap_or(0.0);
        let close = ctx.close().unwrap_or(0.0);
        let date = ctx.datetime().unwrap().format("%d-%m-%Y %H:%M");

        let equity = res
            .metrics
            .iter()
            .map(|x| match x {
                MetricComponentResultUnion::Equity(r) => Some(r),
                _ => None,
            })
            .find(|x| x.is_some())
            .map(|x| x.unwrap());

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

        if !res.metrics.is_empty() {
            let metrics_indentation = " ".repeat(19);
            let metrics_separator = "-".repeat(64);
            let metrics_separator = format!("{metrics_indentation}{metrics_separator}").black();
            println!("{}", metrics_separator);
            for metric in res.metrics {
                match metric {
                    MetricComponentResultUnion::Equity(r) => {
                        println!(
                            "{metrics_indentation}Equity: {}  PnL: {}  Returns: {}",
                            r.equity.round().to_string().bright_magenta(),
                            r.pnl.round().to_string().bright_magenta(),
                            format!("{:0.2}", r.returns).bright_magenta(),
                        );
                    }
                    MetricComponentResultUnion::SharpeRatio(r) => {
                        println!(
                            "{metrics_indentation}Sharpe Ratio: {}",
                            format!("{:0.2}", r).bright_magenta(),
                        );
                    }
                    MetricComponentResultUnion::OmegaRatio(r) => {
                        println!(
                            "{metrics_indentation}Omega Ratio: {}",
                            format!("{:0.2}", r).bright_magenta(),
                        );
                    }
                    MetricComponentResultUnion::TotalClosedTrades(r) => {
                        println!(
                            "{metrics_indentation}Closed trades: {}",
                            format!("{:0.2}", r).bright_magenta(),
                        );
                    }
                    _ => {}
                }
            }
            println!("{}", metrics_separator);
        }
    }

    pub fn run<F: FnMut() -> Option<TradeDirection>>(&mut self, mut cb: F) {
        assert!(!self.already_run, "StrategyRunner can only be run once");
        self.already_run = true;

        let start = self.ctx.get().start_tick();
        let end = self.strategy_end_tick;

        for tick in start..end + 1 {
            self.ctx.get_mutable().current_tick = tick;

            let trade_direction = cb();

            if tick >= self.strategy_start_tick {
                let trade = self.strategy_ctx.next(trade_direction);

                let equity = self.equity.next(trade);

                let mut metrics = vec![MetricComponentResultUnion::Equity(equity)];

                metrics.extend(
                    self.metrics
                        .iter_mut()
                        .map(|r| match r {
                            MetricComponentUnion::OmegaRatio(r) => {
                                MetricComponentResultUnion::OmegaRatio(r.next(&equity))
                            }
                            MetricComponentUnion::SharpeRatio(r) => {
                                MetricComponentResultUnion::SharpeRatio(r.next(&equity))
                            }
                            MetricComponentUnion::TotalClosedTrades(r) => {
                                MetricComponentResultUnion::TotalClosedTrades(r.next(trade))
                            }
                            _ => panic!("Metric not implemented"),
                        })
                        .collect::<Vec<_>>(),
                );

                let res = StrategyRunnerResult { tick, metrics };

                if self.config.print {
                    Self::print(self.ctx.clone(), trade, res);
                }
            }
        }
    }
}
