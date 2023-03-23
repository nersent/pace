#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{
        core::incremental::Incremental,
        ta::{
            average_true_range::Atr, change::Change, exponential_moving_average::Ema,
            highest::Highest,
        },
        testing::{
            array_snapshot::ArraySnapshot,
            fixture::{DataFrameFixtureUtils, Fixture},
            pace::format_pace_fixture_path,
        },
    };

    fn format_path(path: &str) -> PathBuf {
        format_pace_fixture_path(&format!("tests/ta/bars/highest/{}", path))
    }

    fn _test(target: &mut Highest, expected: &[Option<f64>]) {
        let mut snapshot = ArraySnapshot::<Option<f64>>::new();
        for _ in target.ctx.clone() {
            let output = target.next(target.ctx.bar.high());
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn length_14_high() {
        let (_df, ctx) = Fixture::load_ctx(&format_path("length_14_high.csv"));
        _test(&mut Highest::new(ctx.clone(), 14), &_df.test_target());
    }
}
