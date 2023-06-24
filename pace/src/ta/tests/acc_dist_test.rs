#[cfg(test)]
mod tests {

    use std::path::PathBuf;

    use crate::{
        core::incremental::Incremental,
        polars::series::SeriesCastUtils,
        ta::{
            acc_dist::AccDist, average_true_range::Atr, change::Change, cum::Cum,
            exponential_moving_average::Ema, sum::Sum,
        },
        testing::{
            array_snapshot::ArraySnapshot,
            fixture::{DataFrameFixtureUtils, Fixture},
            pace::format_pace_fixture_path,
        },
    };

    fn format_path(path: &str) -> PathBuf {
        format_pace_fixture_path(&format!("tests/ta/acc_dist/{}", path))
    }

    fn _test(target: &mut AccDist, expected: &[f64]) {
        let mut snapshot = ArraySnapshot::<f64>::new();
        for _ in target.ctx.clone() {
            let output = target.next(());
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn default() {
        let (df, ctx) = Fixture::load(&format_path("default.csv"));
        _test(&mut AccDist::new(ctx.clone()), &df.test_target());
    }
}
