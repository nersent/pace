#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{
        core::incremental::Incremental,
        polars::series::SeriesCastUtils,
        ta::{
            exponential_moving_average::Ema, hull_moving_average::Hma,
            linearly_weighted_moving_average::Lwma,
        },
        testing::{
            array_snapshot::ArraySnapshot,
            fixture::{DataFrameFixtureUtils, Fixture},
            pace::format_pace_fixture_path,
        },
    };

    fn format_path(path: &str) -> PathBuf {
        format_pace_fixture_path(&format!("tests/ta/hma/{}", path))
    }

    fn _test(target: &mut Hma, expected: &[f64]) {
        let mut snapshot = ArraySnapshot::<f64>::new();
        for _ in target.ctx.clone() {
            let output = target.next(target.ctx.bar.close());
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn length_2_close() {
        let (df, ctx) = Fixture::load(&format_path("length_2_close.csv"));
        _test(&mut Hma::new(ctx.clone(), 2), &df.test_target());
    }

    #[test]
    fn length_3_close() {
        let (df, ctx) = Fixture::load(&format_path("length_3_close.csv"));
        _test(&mut Hma::new(ctx.clone(), 3), &df.test_target());
    }

    #[test]
    fn length_7_close() {
        let (df, ctx) = Fixture::load(&format_path("length_7_close.csv"));
        _test(&mut Hma::new(ctx.clone(), 7), &df.test_target());
    }

    #[test]
    fn length_14_close() {
        let (df, ctx) = Fixture::load(&format_path("length_14_close.csv"));
        _test(&mut Hma::new(ctx.clone(), 14), &df.test_target());
    }

    #[test]
    fn length_350_close() {
        let (df, ctx) = Fixture::load(&format_path("length_350_close.csv"));
        _test(&mut Hma::new(ctx.clone(), 350), &df.test_target());
    }
}
