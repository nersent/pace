#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{
        core::incremental::Incremental,
        ta::{average_true_range::Atr, exponential_moving_average::Ema},
        testing::{
            array_snapshot::ArraySnapshot,
            fixture::{DataFrameFixtureUtils, Fixture},
            pace::format_pace_fixture_path,
        },
    };

    fn format_path(path: &str) -> PathBuf {
        format_pace_fixture_path(&format!("tests/ta/atr/{}", path))
    }

    fn _test(target: &mut Atr, expected: &[f64]) {
        let mut snapshot = ArraySnapshot::<f64>::new();
        for _ in target.ctx.clone() {
            let output = target.next(());
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn length_1() {
        let (df, ctx) = Fixture::load(&format_path("length_1.csv"));
        _test(&mut Atr::new(ctx.clone(), 1), &df.test_target());
    }

    #[test]
    fn length_2() {
        let (df, ctx) = Fixture::load(&format_path("length_2.csv"));
        _test(&mut Atr::new(ctx.clone(), 2), &df.test_target());
    }

    #[test]
    fn length_14() {
        let (df, ctx) = Fixture::load(&format_path("length_14.csv"));
        _test(&mut Atr::new(ctx.clone(), 14), &df.test_target());
    }
}
