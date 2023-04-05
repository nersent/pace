#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{
        common::src::{Src, SrcKind},
        core::incremental::{Chained, Incremental},
        polars::series::SeriesCastUtils,
        ta::simple_moving_average::Sma,
        testing::{
            array_snapshot::ArraySnapshot,
            fixture::{DataFrameFixtureUtils, Fixture},
            incremental::test_incremental,
            pace::format_pace_fixture_path,
        },
    };

    fn format_path(path: &str) -> PathBuf {
        format_pace_fixture_path(&format!("tests/ta/sma/{}", path))
    }

    fn _test(target: &mut Sma, expected: &[f64]) {
        let mut snapshot = ArraySnapshot::<f64>::new();
        for _ in target.ctx.clone() {
            let output = target.next(target.ctx.bar.close());
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn length_1_close() {
        let (df, ctx) = Fixture::load(&format_path("length_1_close.csv"));
        _test(&mut Sma::new(ctx.clone(), 1), &df.test_target());
    }

    #[test]
    fn length_2_close() {
        let (df, ctx) = Fixture::load(&format_path("length_2_close.csv"));
        _test(&mut Sma::new(ctx.clone(), 2), &df.test_target());
    }

    #[test]
    fn length_3_close() {
        let (df, ctx) = Fixture::load(&format_path("length_3_close.csv"));
        _test(&mut Sma::new(ctx.clone(), 3), &df.test_target());
    }

    #[test]
    fn length_7_close() {
        let (df, ctx) = Fixture::load(&format_path("length_7_close.csv"));
        _test(&mut Sma::new(ctx.clone(), 7), &df.test_target());
    }

    #[test]
    fn length_14_close() {
        let (df, ctx) = Fixture::load(&format_path("length_14_close.csv"));
        _test(&mut Sma::new(ctx.clone(), 14), &df.test_target());
    }

    #[test]
    fn length_350_close() {
        let (df, ctx) = Fixture::load(&format_path("length_350_close.csv"));
        _test(&mut Sma::new(ctx.clone(), 350), &df.test_target());
    }

    // #[test]
    // fn xd_length_2_close() {
    //     let (df, ctx) = Fixture::load(&format_path("length_1_close.csv"));
    //     _test_xd(&mut SmaXd::new(ctx.clone(), 1), &df.test_target());
    // }

    // #[test]
    // fn test_sma_btc_1d_sma_14_length_change_stdev_14_length_close() {
    //     let (_df, ctx, expected) =
    //         Fixture::load("ta/moving_average/tests/fixtures/sma/change/stdev/btc_1d_sma_14_length_change_stdev_14_length_close.csv");
    //     let mut target_sma = SimpleMovingAverageComponent::new(ctx.clone(), 14);
    //     let mut target_stdev = StandardDeviationComponent::new(ctx.clone(), 14, true);
    //     let mut prev_src: f64 = None;
    //     let mut snapshot = ArraySnapshot::<f64::new();
    //     for cctx in ctx {
    //         let ctx = cctx.get();
    //         let src = ctx.close();
    //         let stdev = target_stdev.next(src);
    //         let src_change = match (src, prev_src) {
    //             (Some(src), Some(prev_src)) => Some(src - prev_src),
    //             _ => None,
    //         };
    //         let change: f64 = match src_change {
    //             Some(src_change) => {
    //                 if src_change <= 0.0 {
    //                     Some(0.0)
    //                 } else {
    //                     stdev
    //                 }
    //             }
    //             _ => None,
    //         };
    //         let sma = target_sma.next(change);
    //         prev_src = src;
    //         snapshot.push(sma);
    //     }
    //     snapshot.assert(&expected);
    // }
}
