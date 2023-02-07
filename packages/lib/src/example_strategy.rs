use std::time::Instant;

use crate::{
    base::{
        components::{component_default::ComponentDefault, testing::Fixture},
        strategy::{
            metrics::{
                equity_metric::{EquityMetric, EquityMetricConfig},
                omega_ratio_metric::{OmegaRatioMetric, OmegaRatioMetricConfig},
                sharpe_ratio_metric::{SharpeRatioMetric, SharpeRatioMetricConfig},
                total_closed_trades_metric::TotalClosedTradesMetric,
            },
            strategy_execution_context::{
                StrategyExecutionContext, StrategyExecutionContextConfig,
            },
            trade::TradeDirection,
        },
    },
    content::{
        relative_strength_index_indicator::{
            RelativeStrengthIndexIndicator, RelativeStrengthIndexIndicatorConfig,
        },
        relative_strength_index_strategy::{
            RelativeStrengthIndexStrategy, RelativeStrengthIndexStrategyConfig,
        },
    },
};
use colored::Colorize;

pub fn run_example_strategy() -> u128 {
    let (df, ctx) = Fixture::raw("base/strategy/tests/fixtures/btc_1d.csv");

    let mut strategy_ctx = StrategyExecutionContext::new(
        ctx.clone(),
        StrategyExecutionContextConfig {
            on_bar_close: false,
            continous: false,
        },
    );

    let mut rsi_indicator = RelativeStrengthIndexIndicator::new(
        ctx.clone(),
        RelativeStrengthIndexIndicatorConfig::default(ctx.clone()),
    );

    let mut rsi_strategy = RelativeStrengthIndexStrategy::new(
        ctx.clone(),
        RelativeStrengthIndexStrategyConfig::default(ctx.clone()),
    );

    let mut equity_metric = EquityMetric::new(
        ctx.clone(),
        EquityMetricConfig {
            initial_capital: 1000.0,
        },
    );
    let mut sharpe_ratio_metric = SharpeRatioMetric::new(
        ctx.clone(),
        SharpeRatioMetricConfig {
            risk_free_rate: 0.0,
        },
    );
    let mut omega_ratio_metric = OmegaRatioMetric::new(
        ctx.clone(),
        OmegaRatioMetricConfig {
            risk_free_rate: 0.0,
        },
    );
    let mut total_closed_trades_metric = TotalClosedTradesMetric::new(ctx.clone());

    let start_time = Instant::now();

    for cctx in ctx {
        let ctx = cctx.get();
        let tick = ctx.current_tick;
        let time = ctx.time();

        let mut trade: Option<TradeDirection> = None;

        if false {
            let rsi = rsi_indicator.next();
            let rsi_trade = rsi_strategy.next(rsi);

            trade = rsi_trade;
        } else {
            let long_ticks = [2, 20];
            let short_ticks = [10, 15];

            if long_ticks.contains(&tick) {
                trade = Some(TradeDirection::Long);
            } else if short_ticks.contains(&tick) {
                trade = Some(TradeDirection::Short);
            }
        }

        let current_trade = strategy_ctx.next(trade);
        let equity = equity_metric.next(current_trade);
        let sharpe_ratio = sharpe_ratio_metric.next(&equity) * f64::sqrt(365.0);
        let omega_ratio = omega_ratio_metric.next(&equity) * f64::sqrt(365.0);
        let total_closed_trades = total_closed_trades_metric.next(current_trade);

        println!(
            "\n{}: {}{} | {}\n{}\n{}\n{}\n{}",
            format!("[{}]", tick).bright_cyan().bold(),
            format!("{:?}", ctx.close().unwrap_or(0.0)).blue(),
            format!(" | {}", current_trade.map(|x| x.to_colored_string(tick)).unwrap_or("No trade".bright_black())).to_string(),
            format!(
                "{}",
                 ctx.datetime().unwrap().format("%d-%m-%Y %H:%M")
            )
            .bright_black(),
            format!(
                "Equity: {:0.2} | Returns: {:0.2} | Mean returns: {:0.2} | Stdev Returns: {:0.2} | Fill size: {:?} | pnL: {} | Trade pnL: {:0.2} | Fixed Returns: {:0.2}",
                equity.equity, equity.returns, equity.returns_mean, equity.returns_stdev, equity_metric.trade_fill_size, equity.pnl, equity.trade_pnl, equity.fixed_returns
            )
            .bright_black(),
            format!("Sharpe: {:0.2}", sharpe_ratio).bright_black(),
            format!("Omega: {:0.2}", omega_ratio).bright_black(),
            format!("Total closed trades: {}", total_closed_trades).bright_black(),

        );

        if (tick > 100) {
            break;
        }
    }

    let end_time = Instant::now();
    let elapsed_time = end_time - start_time;
    let elapsed_time = elapsed_time.as_micros();

    return elapsed_time;
}
