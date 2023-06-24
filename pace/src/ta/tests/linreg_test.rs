#[cfg(test)]
mod tests {

    use std::path::PathBuf;

    use crate::{
        core::incremental::Incremental,
        polars::series::SeriesCastUtils,
        ta::{
            average_true_range::Atr, change::Change, cum::Cum, exponential_moving_average::Ema,
            linreg::Linreg, sum::Sum,
        },
        testing::{
            array_snapshot::ArraySnapshot,
            fixture::{DataFrameFixtureUtils, Fixture},
            pace::format_pace_fixture_path,
        },
    };

    fn format_path(path: &str) -> PathBuf {
        format_pace_fixture_path(&format!("tests/ta/linreg/{}", path))
    }

    fn _test(target: &mut Linreg, expected: &[f64]) {
        let mut snapshot = ArraySnapshot::<f64>::new();
        for _ in target.ctx.clone() {
            let output = target.next(target.ctx.bar.close());
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn close_length_20_offset_0() {
        let (df, ctx) = Fixture::load(&format_path("close_length_20_offset_0.csv"));
        _test(&mut Linreg::new(ctx.clone(), 20, 0), &df.test_target());
    }
}
