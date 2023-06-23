#[cfg(test)]
mod tests {

    use std::path::PathBuf;

    use crate::{
        core::incremental::Incremental,
        polars::series::SeriesCastUtils,
        ta::{
            average_true_range::Atr, change::Change, cum::Cum, exponential_moving_average::Ema,
            price_volume_trend::Pvt, sum::Sum,
        },
        testing::{
            array_snapshot::ArraySnapshot,
            fixture::{DataFrameFixtureUtils, Fixture},
            pace::format_pace_fixture_path,
        },
    };

    fn format_path(path: &str) -> PathBuf {
        format_pace_fixture_path(&format!("tests/ta/pvt/{}", path))
    }

    fn _test(target: &mut Pvt, expected: &[f64]) {
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
        _test(&mut Pvt::new(ctx.clone()), &df.test_target());
    }
}
