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
use strategy::strategy_context::{StrategyContext, StrategyContextConfig};

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

    for cctx in ctx {
        let ctx = cctx.get();
        let current_tick = ctx.tick();
        let current_price = ctx.open();
        let mut action: StrategyActionKind = StrategyActionKind::None;

        if (current_tick == 4 || current_tick == 5) {
            action = StrategyActionKind::Long;
        } else if (current_tick == 8) {
            action = StrategyActionKind::Short;
        }

        strategy.next(action);

        println!(
            "\n{} {} | {}\n{:?}",
            format!("[{}]", current_tick).bright_cyan().bold(),
            format!("{:?}", current_price).bright_black(),
            match action {
                StrategyActionKind::None => format!("None").bright_white(),
                StrategyActionKind::Long => format!("▲ [Long]").bright_green().bold(),
                StrategyActionKind::Short => format!("▲ [Short]").bright_red().bold(),
            },
            strategy.trades.last()
        );

        if (current_tick > 10) {
            break;
        }
    }
}

fn main() {
    example_strategy();
    // generate_ml_dataset();
}
