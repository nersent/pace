#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{
        core::incremental::Incremental,
        ta::running_moving_average::Rma,
        testing::{
            array_snapshot::ArraySnapshot,
            fixture::{DataFrameFixtureUtils, Fixture},
            pace::format_pace_fixture_path,
        },
    };

    fn format_path(path: &str) -> PathBuf {
        format_pace_fixture_path(&format!("tests/ta/rma/{}", path))
    }

    fn _test(target: &mut Rma, expected: &[f64]) {
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
        _test(&mut Rma::new(ctx.clone(), 1), &df.test_target());
    }

    #[test]
    fn length_2_close() {
        let (df, ctx) = Fixture::load(&format_path("length_2_close.csv"));
        _test(&mut Rma::new(ctx.clone(), 2), &df.test_target());
    }

    #[test]
    fn length_3_close() {
        let (df, ctx) = Fixture::load(&format_path("length_3_close.csv"));
        _test(&mut Rma::new(ctx.clone(), 3), &df.test_target());
    }

    #[test]
    fn length_7_close() {
        let (df, ctx) = Fixture::load(&format_path("length_7_close.csv"));
        _test(&mut Rma::new(ctx.clone(), 7), &df.test_target());
    }

    #[test]
    fn length_14_close() {
        let (df, ctx) = Fixture::load(&format_path("length_14_close.csv"));
        _test(&mut Rma::new(ctx.clone(), 14), &df.test_target());
    }

    #[test]
    fn length_350_close() {
        let (df, ctx) = Fixture::load(&format_path("length_350_close.csv"));
        _test(&mut Rma::new(ctx.clone(), 350), &df.test_target());
    }
}
