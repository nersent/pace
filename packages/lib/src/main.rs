#![allow(
    clippy::needless_return,
    clippy::type_complexity,
    clippy::needless_range_loop,
    clippy::too_many_arguments,
    clippy::uninlined_format_args,
    unused
)]

use std::path::Path;

use base::{
    component_context::ComponentContext,
    features::{feature::Feature, feature_regions::FeatureTernaryTrendRegions},
    strategy::types::StrategyActionKind,
    utils::testing::{get_test_artifact_path, load_test_artifact},
};
use data::{types::Timeframe, utils::read_csv};

use crate::base::features::{feature::FeatureNamespace, feature_composer::FeatureComposer};

mod base;
mod data;
mod dataset;
mod features;
mod indicators;
mod strategies;
mod utils;

fn generate_ml_dataset() {
    let df = read_csv(Path::new(
        "artifacts/tests/implicit/recursive/sma/btc_1d_length_2_close.csv",
    ));
    let ctx = ComponentContext::build_from_df(&df, "BTC_USD", Timeframe::OneDay);
    dataset::dataset_ml::generate_ml_dataset(ctx, Path::new(".out/dataset_ml.csv"));
    println!("[process] exit");
}

fn main() {
    generate_ml_dataset();
}
