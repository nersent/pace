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

use crate::{
    asset::timeframe::Timeframe, components::component_context::ComponentContext,
    data::csv::read_csv, testing::fixture::Fixture,
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

fn main() {
    generate_ml_dataset();
}
