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

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

mod base;
mod content;
mod example_strategy;
mod ml;
mod utils;

use std::path::Path;

use ml::dataset_ml::generate_ml_datasets;

use crate::base::components::testing::Fixture;

fn benchmark_example_strategy() {
    let mut times: Vec<u128> = vec![];

    let max = 2000;
    for i in 0..max {
        if i % 100 == 0 {
            println!("{:0.2}%", (i as f64) / (max as f64) * 100.0);
        }
        let time = example_strategy::run_example_strategy();
        times.push(time);
    }

    let mean = times.iter().sum::<u128>() as f64 / times.len() as f64;

    println!("{}", mean);
}

fn main() {
    example_strategy::run_example_strategy_refactor();
    // benchmark_example_strategy();
    // example_strategy::run_example_strategy();
    // generate_ml_datasets();
}
