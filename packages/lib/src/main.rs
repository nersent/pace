#![allow(
    clippy::needless_return,
    clippy::type_complexity,
    clippy::needless_range_loop,
    unused
)]

use base::{features::feature::Feature, strategy::types::StrategyActionKind};
use features::feature_composer::FeatureComposer;
use indicators::indicator_rsi::{INDICATOR_RSI_MAX_VALUE, INDICATOR_RSI_MIN_VALUE};

use crate::features::utils::compute_regions;

mod base;
mod data;
mod features;
mod indicators;
mod strategies;
mod utils;

fn main() {
    let mut composer = FeatureComposer::new();
    let rsi = Some(75.0);
    let root = Feature::as_root(
        "rsi",
        Vec::from([
            Feature::as_raw("value", rsi),
            Feature::from_strategy_action(Some(StrategyActionKind::Long)),
            Feature::to_overbought_oversold_regions(
                "value",
                rsi,
                INDICATOR_RSI_MIN_VALUE,
                INDICATOR_RSI_MAX_VALUE,
                30.0,
                70.0,
            ),
            Feature::as_numeric("up", Some(0.84)),
            Feature::as_numeric("down", Some(0.14)),
        ]),
    );
    composer.push_row(Vec::from([root]));
    let rsi = Some(0.0);
    let root = Feature::as_root(
        "rsi",
        Vec::from([
            Feature::as_raw("value", rsi),
            Feature::from_strategy_action(Some(StrategyActionKind::Long)),
            Feature::to_overbought_oversold_regions(
                "value",
                rsi,
                INDICATOR_RSI_MIN_VALUE,
                INDICATOR_RSI_MAX_VALUE,
                30.0,
                70.0,
            ),
            Feature::as_numeric("up", Some(0.84)),
            Feature::as_numeric("down", Some(0.14)),
        ]),
    );
    composer.push_row(Vec::from([root]));
    let rsi = Some(100.0);
    let root = Feature::as_root(
        "rsi",
        Vec::from([
            Feature::as_raw("value", rsi),
            Feature::from_strategy_action(Some(StrategyActionKind::Long)),
            Feature::to_overbought_oversold_regions(
                "value",
                rsi,
                INDICATOR_RSI_MIN_VALUE,
                INDICATOR_RSI_MAX_VALUE,
                30.0,
                70.0,
            ),
            Feature::as_numeric("up", Some(0.84)),
            Feature::as_numeric("down", Some(0.14)),
        ]),
    );
    composer.push_row(Vec::from([root]));
    let rsi = Some(50.0);
    let root = Feature::as_root(
        "rsi",
        Vec::from([
            Feature::as_raw("value", rsi),
            Feature::from_strategy_action(Some(StrategyActionKind::Long)),
            Feature::to_overbought_oversold_regions(
                "value",
                rsi,
                INDICATOR_RSI_MIN_VALUE,
                INDICATOR_RSI_MAX_VALUE,
                30.0,
                70.0,
            ),
            Feature::as_numeric("up", Some(0.84)),
            Feature::as_numeric("down", Some(0.14)),
        ]),
    );
    composer.push_row(Vec::from([root]));

    let map = composer.flatten();
    println!("{:?}", map);
}
