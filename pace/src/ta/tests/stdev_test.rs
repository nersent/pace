#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{
        core::incremental::Incremental,
        polars::series::SeriesCastUtils,
        ta::{
            average_true_range::Atr, change::Change, exponential_moving_average::Ema, stdev::Stdev,
        },
        testing::{
            array_snapshot::ArraySnapshot,
            fixture::{DataFrameFixtureUtils, Fixture},
            pace::format_pace_fixture_path,
        },
    };

    fn format_path(path: &str) -> PathBuf {
        format_pace_fixture_path(&format!("tests/ta/stdev/{}", path))
    }

    fn _test(target: &mut Stdev, expected: &[f64]) {
        let mut snapshot = ArraySnapshot::<f64>::new();
        for _ in target.ctx.clone() {
            let output = target.next(target.ctx.bar.close());
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn unbiased_length_1_close() {
        let (df, ctx) = Fixture::load(&format_path("length_1_unbiased.csv"));
        _test(&mut Stdev::new(ctx.clone(), 1, false), &df.test_target());
    }

    #[test]
    fn unbiased_length_2_close() {
        let (df, ctx) = Fixture::load(&format_path("length_2_unbiased.csv"));
        _test(&mut Stdev::new(ctx.clone(), 2, false), &df.test_target());
    }

    #[test]
    fn unbiased_length_3_close() {
        let (df, ctx) = Fixture::load(&format_path("length_3_unbiased.csv"));
        _test(&mut Stdev::new(ctx.clone(), 3, false), &df.test_target());
    }

    #[test]
    fn unbiased_length_14_close() {
        let (df, ctx) = Fixture::load(&format_path("length_14_unbiased.csv"));
        _test(&mut Stdev::new(ctx.clone(), 14, false), &df.test_target());
    }

    #[test]
    fn unbiased_length_365_close() {
        let (df, ctx) = Fixture::load(&format_path("length_365_unbiased.csv"));
        _test(&mut Stdev::new(ctx.clone(), 365, false), &df.test_target());
    }

    #[test]
    fn biased_length_1_close() {
        let (df, ctx) = Fixture::load(&format_path("length_1_biased.csv"));
        _test(&mut Stdev::new(ctx.clone(), 1, true), &df.test_target());
    }

    #[test]
    fn biased_length_2_close() {
        let (df, ctx) = Fixture::load(&format_path("length_2_biased.csv"));
        _test(&mut Stdev::new(ctx.clone(), 2, true), &df.test_target());
    }

    #[test]
    fn biased_length_3_close() {
        let (df, ctx) = Fixture::load(&format_path("length_3_biased.csv"));
        _test(&mut Stdev::new(ctx.clone(), 3, true), &df.test_target());
    }

    #[test]
    fn biased_length_14_close() {
        let (df, ctx) = Fixture::load(&format_path("length_14_biased.csv"));
        _test(&mut Stdev::new(ctx.clone(), 14, true), &df.test_target());
    }

    #[test]
    fn biased_length_365_close() {
        let (df, ctx) = Fixture::load(&format_path("length_365_biased.csv"));
        _test(&mut Stdev::new(ctx.clone(), 365, true), &df.test_target());
    }
}
