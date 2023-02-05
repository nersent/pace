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

use std::path::Path;

use crate::base::components::testing::Fixture;

mod base;
mod content;
mod ml;
mod utils;

fn generate_ml_dataset() {
    let (df, ctx) = Fixture::raw("ml/fixtures/btc_1d.csv");
    ml::dataset_ml::generate_ml_dataset(ctx, Path::new(".out/dataset_ml.csv"));
    println!("[process] exit");
}

fn main() {
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

    generate_ml_dataset();
}
