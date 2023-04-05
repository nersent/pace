#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{
        core::incremental::Incremental,
        polars::series::SeriesCastUtils,
        ta::{average_true_range::Atr, change::Change, exponential_moving_average::Ema},
        testing::{
            array_snapshot::ArraySnapshot,
            fixture::{DataFrameFixtureUtils, Fixture},
            pace::format_pace_fixture_path,
        },
    };

    fn format_path(path: &str) -> PathBuf {
        format_pace_fixture_path(&format!("tests/ta/change/{}", path))
    }

    fn _test(target: &mut Change, expected: &[f64]) {
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
        _test(&mut Change::new(ctx.clone(), 1), &df.test_target());
    }

    #[test]
    fn length_2_close() {
        let (df, ctx) = Fixture::load(&format_path("length_2_close.csv"));
        _test(&mut Change::new(ctx.clone(), 2), &df.test_target());
    }

    #[test]
    fn length_3_close() {
        let (df, ctx) = Fixture::load(&format_path("length_3_close.csv"));
        _test(&mut Change::new(ctx.clone(), 3), &df.test_target());
    }

    #[test]
    fn length_365_close() {
        let (df, ctx) = Fixture::load(&format_path("length_365_close.csv"));
        _test(&mut Change::new(ctx.clone(), 365), &df.test_target());
    }
}
