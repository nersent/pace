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
mod utils;

use std::path::Path;

use ml::dataset_ml::generate_ml_datasets;

use crate::base::components::testing::Fixture;

fn main() {
    // example_strategy::run_example_strategy();
    // let (_df, cctx, expected) =
    //     Fixture::load("components/change/tests/fixtures/prank/btc_1d_length_14_close.csv");
    // let mut target = RecursivePercentRank::new(cctx.clone(), 14);
    // for cctx in cctx {
    //     let ctx = cctx.get();
    //     let output = target.next(ctx.close());
    //     println!(
    //         "[{}]: {:?} | {:?}",
    //         ctx.current_tick, output, expected[ctx.current_tick]
    //     );
    //     if ctx.current_tick > 35 {
    //         break;
    //     }
    // }

    generate_ml_datasets();
}
