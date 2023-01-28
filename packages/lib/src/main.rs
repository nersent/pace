#![allow(
    clippy::needless_return,
    clippy::type_complexity,
    clippy::needless_range_loop,
    clippy::too_many_arguments,
    clippy::uninlined_format_args,
    clippy::module_inception,
    unused
)]

use std::path::Path;

use colored::Colorize;
use polars::export::chrono::format;
use strategy::{
    metrics::{
        strategy_equity_metric::{StrategyEquityMetric, StrategyEquityMetricConfig},
        strategy_sharpe_ratio_metric::{
            StrategySharpeRatioMetric, StrategySharpeRatioMetricConfig,
        },
    },
    strategy_context::{StrategyContext, StrategyContextConfig},
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

fn example_strategy() {
    let (df, ctx) = Fixture::raw("strategy/tests/fixtures/example.csv");
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
            multiplier: 1.0,
        },
    );

    for cctx in ctx {
        let ctx = cctx.get();
        let tick = ctx.tick();
        let price = ctx.open();
        let mut action: StrategyActionKind = StrategyActionKind::None;

        let long_ticks = [5];
        let short_ticks = [1];

        if long_ticks.contains(&tick) {
            action = StrategyActionKind::Long;
        } else if short_ticks.contains(&tick) {
            action = StrategyActionKind::Short;
        }

        // if (current_tick == 4 || current_tick == 7) {
        //     action = StrategyActionKind::Long;
        // } else if (current_tick == 10 || current_tick == 14) {
        //     action = StrategyActionKind::Short;
        // }

        let current_trade = strategy.next(action);
        let equity = equity.next(current_trade);
        let sharpe_ratio = sharpe_ratio.next(equity);

        println!(
            "\n{}: {}{}\n{}\n{}",
            format!("[{}]", tick).bright_cyan().bold(),
            format!("{:?}", price.unwrap_or(0.0)).blue(),
            if current_trade.is_none() || current_trade.unwrap().entry_price.is_none() {
                "".to_string()
            } else {
                format!("| {}", current_trade.unwrap().to_colored_string()).to_string()
            },
            format!(
                "Equity: {:0.2} | Returns: {:0.2} | Mean returns: {:0.2} | Stdev Returns: {:0.2}",
                equity.equity, equity.returns, equity.returns_mean, equity.returns_stdev
            )
            .bright_black(),
            format!("Sharpe: {:0.2}", sharpe_ratio).bright_black(),
            // current_trade,
        );

        if (tick > 20) {
            break;
        }
    }
}

fn main() {
    example_strategy();
    // generate_ml_dataset();
}
