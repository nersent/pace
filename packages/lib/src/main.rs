#![allow(
    clippy::needless_return,
    clippy::type_complexity,
    clippy::needless_range_loop,
    clippy::too_many_arguments,
    clippy::uninlined_format_args,
    clippy::module_inception,
    unused
)]

use std::{path::Path, time::Instant};

use chrono::{DateTime, NaiveDateTime};
use colored::Colorize;
use components::source::{Source, SourceKind};
use strategy::{
    metrics::{
        strategy_equity_metric::{StrategyEquityMetric, StrategyEquityMetricConfig},
        strategy_omega_ratio_metric::{StrategyOmegaRatioMetric, StrategyOmegaRatioMetricConfig},
        strategy_sharpe_ratio_metric::{
            StrategySharpeRatioMetric, StrategySharpeRatioMetricConfig,
        },
        strategy_total_closed_trades_metric::StrategyTotalClosedTradesMetric,
    },
    strategy_context::{StrategyContext, StrategyContextConfig},
};
use ta::relative_strength_index::{
    rsi_indicator::{
        RelativeStrengthIndexIndicator, RelativeStrengthIndexIndicatorConfig,
        RelativeStrengthIndexIndicatorResult,
    },
    rsi_strategy::{
        RelativeStrengthIndexStrategy, RelativeStrengthIndexStrategyConfig,
        RSI_STRATEGY_THRESHOLD_OVERBOUGHT, RSI_STRATEGY_THRESHOLD_OVERSOLD,
    },
};

use crate::{
    asset::timeframe::Timeframe, components::component_context::ComponentContext,
    data::csv::read_csv, strategy::action::StrategyActionKind, testing::fixture::Fixture,
};
mod asset;
mod components;
mod data;
mod features;
mod math;
mod ml;
mod strategy;
mod ta;
mod testing;
mod utils;

fn generate_ml_dataset() {
    let (df, ctx) = Fixture::raw("ml/fixtures/btc_1d.csv");
    ml::dataset_ml::generate_ml_dataset(ctx, Path::new(".out/dataset_ml.csv"));
    println!("[process] exit");
}

fn example_strategy() -> u128 {
    let (df, ctx) = Fixture::raw("strategy/tests/fixtures/example.csv");

    let mut rsi_strategy = RelativeStrengthIndexStrategy::new(
        ctx.clone(),
        RelativeStrengthIndexStrategyConfig {
            threshold_oversold: RSI_STRATEGY_THRESHOLD_OVERSOLD,
            threshold_overbought: RSI_STRATEGY_THRESHOLD_OVERBOUGHT,
        },
        RelativeStrengthIndexIndicator::new(
            ctx.clone(),
            RelativeStrengthIndexIndicatorConfig {
                length: 14,
                src: Source::from_kind(ctx.clone(), SourceKind::Close),
            },
        ),
    );

    let mut strategy = StrategyContext::new(
        ctx.clone(),
        StrategyContextConfig {
            on_bar_close: false,
        },
    );

    let mut equity = StrategyEquityMetric::new(
        ctx.clone(),
        StrategyEquityMetricConfig {
            initial_capital: 1000.0,
        },
    );
    let mut sharpe_ratio = StrategySharpeRatioMetric::new(
        ctx.clone(),
        StrategySharpeRatioMetricConfig {
            risk_free_rate: 0.0,
        },
    );
    let mut omega_ratio = StrategyOmegaRatioMetric::new(
        ctx.clone(),
        StrategyOmegaRatioMetricConfig {
            risk_free_rate: 0.0,
        },
    );
    let mut total_closed_trades = StrategyTotalClosedTradesMetric::new(ctx.clone());
    let start_time = Instant::now();

    for cctx in ctx {
        let ctx = cctx.get();
        let tick = ctx.tick();
        let price = ctx.open();
        let time = ctx.time();
        let mut action: StrategyActionKind = StrategyActionKind::None;

        let long_ticks = [];
        let short_ticks = [];

        if long_ticks.contains(&tick) {
            action = StrategyActionKind::Long;
        } else if short_ticks.contains(&tick) {
            action = StrategyActionKind::Short;
        }

        let (rsi_action, _) = rsi_strategy.next();
        action = rsi_action;

        // if (current_tick == 4 || current_tick == 7) {
        //     action = StrategyActionKind::Long;
        // } else if (current_tick == 10 || current_tick == 14) {
        //     action = StrategyActionKind::Short;
        // }

        let current_trade = strategy.next(action);
        let equity = equity.next(current_trade);
        let sharpe_ratio = sharpe_ratio.next(equity) * f64::sqrt(365.0);
        let omega_ratio = omega_ratio.next(equity) * f64::sqrt(365.0);
        let total_closed_trades = total_closed_trades.next(current_trade);

        // println!(
        //     "\n{}: {}{} | {}\n{}\n{}\n{}\n{}",
        //     format!("[{}]", tick).bright_cyan().bold(),
        //     format!("{:?}", price.unwrap_or(0.0)).blue(),
        //     if current_trade.is_none() || current_trade.unwrap().entry_price.is_none() {
        //         "".to_string()
        //     } else {
        //         format!("| {}", current_trade.unwrap().to_colored_string()).to_string()
        //     },
        //     format!(
        //         "{}",
        //         NaiveDateTime::from_timestamp_millis(time.unwrap().as_millis() as i64)
        //             .unwrap()
        //             .format("%d-%m-%Y %H:%M")
        //     )
        //     .bright_black(),
        //     format!(
        //         "Equity: {:0.2} | Returns: {:0.2} | Mean returns: {:0.2} | Stdev Returns: {:0.2}",
        //         equity.equity, equity.returns, equity.returns_mean, equity.returns_stdev
        //     )
        //     .bright_black(),
        //     format!("Sharpe: {:0.2}", sharpe_ratio).bright_black(),
        //     format!("Omega: {:0.2}", omega_ratio).bright_black(),
        //     format!("Total closed trades: {}", total_closed_trades).bright_black(),
        //     // current_trade,
        // );

        // if (tick > 450) {
        //     break;
        // }
    }

    let end_time = Instant::now();
    let elapsed_time = end_time - start_time;
    let elapsed_time = elapsed_time.as_micros();

    return elapsed_time;
}

fn main() {
    let mut time_sum: u128 = 0;
    let mut time_count: u128 = 0;
    let max = 5000;
    for i in 0..max {
        let elapsed_time = example_strategy();
        time_sum += elapsed_time;
        time_count += 1;
        if time_count % 250 == 0 {
            println!("{}%", f64::round((i as f64 / max as f64) * 100.0));
        }
    }

    let mean = time_sum / time_count;

    println!("Mean runtime: {}µs", mean);

    // generate_ml_dataset();
}
