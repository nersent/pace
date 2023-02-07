#![allow(
    clippy::needless_return,
    clippy::type_complexity,
    clippy::needless_range_loop,
    clippy::too_many_arguments,
    clippy::uninlined_format_args,
    clippy::module_inception,
    clippy::upper_case_acronyms,
    unused
)]

mod base;
mod content;
mod example_strategy;
mod ml;
mod python;
mod utils;

use std::path::Path;

use ml::dataset_ml::generate_ml_datasets;

use crate::base::components::testing::Fixture;

fn main() {
    example_strategy::run_example_strategy();
    // generate_ml_datasets();
}
