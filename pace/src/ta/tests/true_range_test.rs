#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{
        core::incremental::Incremental,
        ta::{
            average_true_range::Atr, change::Change, exponential_moving_average::Ema,
            true_range::Tr,
        },
        testing::{
            array_snapshot::ArraySnapshot,
            fixture::{DataFrameFixtureUtils, Fixture},
            pace::format_pace_fixture_path,
        },
    };

    fn format_path(path: &str) -> PathBuf {
        format_pace_fixture_path(&format!("tests/ta/tr/{}", path))
    }

    fn _test(target: &mut Tr, expected: &[f64]) {
        let mut snapshot = ArraySnapshot::<f64>::new();
        for _ in target.ctx.clone() {
            let output = target.next(());
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn without_handle_na() {
        let (df, ctx) = Fixture::load(&format_path("without_handle.csv"));
        _test(&mut Tr::new(ctx.clone(), false), &df.test_target());
    }

    #[test]
    fn with_handle_na() {
        let (df, ctx) = Fixture::load(&format_path("with_handle.csv"));
        _test(&mut Tr::new(ctx.clone(), true), &df.test_target());
    }
}
