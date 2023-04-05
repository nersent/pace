#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{
        core::incremental::Incremental,
        polars::series::SeriesCastUtils,
        ta::{
            average_true_range::Atr, change::Change, exponential_moving_average::Ema,
            percent_rank::Prank, rate_of_change::Roc,
        },
        testing::{
            array_snapshot::ArraySnapshot,
            fixture::{DataFrameFixtureUtils, Fixture},
            pace::format_pace_fixture_path,
        },
    };

    fn format_path(path: &str) -> PathBuf {
        format_pace_fixture_path(&format!("tests/ta/prank/{}", path))
    }

    fn _test(target: &mut Prank, expected: &[f64]) {
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
        _test(&mut Prank::new(ctx.clone(), 1), &df.test_target());
    }

    #[test]
    fn length_2_close() {
        let (df, ctx) = Fixture::load(&format_path("length_2_close.csv"));
        _test(&mut Prank::new(ctx.clone(), 2), &df.test_target());
    }

    #[test]
    fn length_3_close() {
        let (df, ctx) = Fixture::load(&format_path("length_3_close.csv"));
        _test(&mut Prank::new(ctx.clone(), 3), &df.test_target());
    }

    #[test]
    fn length_14_close() {
        let (df, ctx) = Fixture::load(&format_path("length_14_close.csv"));
        _test(&mut Prank::new(ctx.clone(), 14), &df.test_target());
    }

    #[test]
    fn length_100_close() {
        let (df, ctx) = Fixture::load(&format_path("length_100_close.csv"));
        _test(&mut Prank::new(ctx.clone(), 100), &df.test_target());
    }

    #[test]
    fn length_365_close() {
        let (df, ctx) = Fixture::load(&format_path("length_365_close.csv"));
        _test(&mut Prank::new(ctx.clone(), 365), &df.test_target());
    }

    fn _test_with_roc(target: &mut Prank, target_roc: &mut Roc, expected: &[f64]) {
        let mut snapshot = ArraySnapshot::<f64>::new();
        for _ in target.ctx.clone() {
            let output_roc = target_roc.next(target.ctx.bar.close());
            let output = target.next(output_roc);
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn length_14_with_roc_length_7_close() {
        let (df, ctx) = Fixture::load(&format_path("roc/length_14_with_roc_length_7_close.csv"));
        _test_with_roc(
            &mut Prank::new(ctx.clone(), 14),
            &mut Roc::new(ctx.clone(), 7),
            &df.test_target(),
        );
    }

    #[test]
    fn length_100_with_roc_length_7_close() {
        let (df, ctx) = Fixture::load(&format_path("roc/length_100_with_roc_length_7_close.csv"));
        _test_with_roc(
            &mut Prank::new(ctx.clone(), 100),
            &mut Roc::new(ctx.clone(), 7),
            &df.test_target(),
        );
    }

    #[test]
    fn length_100_with_roc_length_2_close() {
        let (df, ctx) = Fixture::load(&format_path("roc/length_100_with_roc_length_2_close.csv"));
        _test_with_roc(
            &mut Prank::new(ctx.clone(), 100),
            &mut Roc::new(ctx.clone(), 2),
            &df.test_target(),
        );
    }

    #[test]
    fn length_100_with_roc_length_1_close() {
        let (df, ctx) = Fixture::load(&format_path("roc/length_100_with_roc_length_1_close.csv"));
        _test_with_roc(
            &mut Prank::new(ctx.clone(), 100),
            &mut Roc::new(ctx.clone(), 1),
            &df.test_target(),
        );
    }

    // #[test]
    // fn test_recursive_rank_7_length_with_rate_of_change_14_length_btc_1d_close() {
    //     let (_df, ctx, expected) = Fixture::load(
    //         "s/change/tests/fixtures/prank/roc/btc_1d_prank_length_7_roc_length_14_close.csv",
    //     );
    //     _test_with_roc(
    //         &mut ctx.clone(),
    //         &mut RecursivePercentRank::new(ctx.clone(), 7),
    //         &mut RecursiveRateOfChange::new(ctx.clone(), 14),
    //         &expected,
    //     );
    // }

    // #[test]
    // fn test_recursive_rank_7_length_with_rate_of_change_1_length_btc_1d_close() {
    //     let (_df, ctx, expected) = Fixture::load(
    //         "s/change/tests/fixtures/prank/roc/btc_1d_prank_length_7_roc_length_1_close.csv",
    //     );
    //     _test_with_roc(
    //         &mut ctx.clone(),
    //         &mut RecursivePercentRank::new(ctx.clone(), 7),
    //         &mut RecursiveRateOfChange::new(ctx.clone(), 1),
    //         &expected,
    //     );
    // }
}
