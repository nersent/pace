#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{
        core::incremental::Incremental,
        ta::{
            average_true_range::Atr, change::Change, exponential_moving_average::Ema, stoch::Stoch,
        },
        testing::{
            array_snapshot::ArraySnapshot,
            fixture::{DataFrameFixtureUtils, Fixture},
            pace::format_pace_fixture_path,
        },
    };

    fn format_path(path: &str) -> PathBuf {
        format_pace_fixture_path(&format!("tests/ta/stoch/{}", path))
    }

    fn _test_close_high_low(target: &mut Stoch, expected: &[f64]) {
        let mut snapshot = ArraySnapshot::<f64>::new();
        for _ in target.ctx.clone() {
            let ouptut = target.next((
                target.ctx.bar.close(),
                target.ctx.bar.high(),
                target.ctx.bar.low(),
            ));
            snapshot.push(ouptut);
        }
        snapshot.assert(expected);
    }

    fn _test_close_close_close(target: &mut Stoch, expected: &[f64]) {
        let mut snapshot = ArraySnapshot::<f64>::new();
        for _ in target.ctx.clone() {
            let ouptut = target.next((
                target.ctx.bar.close(),
                target.ctx.bar.close(),
                target.ctx.bar.close(),
            ));
            snapshot.push(ouptut);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn length_14_close_high_low() {
        let (df, ctx) = Fixture::load(&format_path("length_14_close_high_low.csv"));
        _test_close_high_low(&mut Stoch::new(ctx.clone(), 14), &df.test_target());
    }

    #[test]
    fn length_1_close_high_low() {
        let (df, ctx) = Fixture::load(&format_path("length_1_close_high_low.csv"));
        _test_close_high_low(&mut Stoch::new(ctx.clone(), 1), &df.test_target());
    }

    #[test]
    fn length_2_close_high_low() {
        let (df, ctx) = Fixture::load(&format_path("length_2_close_high_low.csv"));
        _test_close_high_low(&mut Stoch::new(ctx.clone(), 2), &df.test_target());
    }

    #[test]
    fn length_3_close_high_low() {
        let (df, ctx) = Fixture::load(&format_path("length_3_close_high_low.csv"));
        _test_close_high_low(&mut Stoch::new(ctx.clone(), 3), &df.test_target());
    }

    #[test]
    fn length_1_close_close_close() {
        let (df, ctx) = Fixture::load(&format_path("length_1_close_close_close.csv"));
        _test_close_close_close(&mut Stoch::new(ctx.clone(), 1), &df.test_target());
    }

    #[test]
    fn length_2_close_close_close() {
        let (df, ctx) = Fixture::load(&format_path("length_2_close_close_close.csv"));
        _test_close_close_close(&mut Stoch::new(ctx.clone(), 2), &df.test_target());
    }

    #[test]
    fn length_3_close_close_close() {
        let (df, ctx) = Fixture::load(&format_path("length_3_close_close_close.csv"));
        _test_close_close_close(&mut Stoch::new(ctx.clone(), 3), &df.test_target());
    }

    #[test]
    fn length_14_close_close_close() {
        let (df, ctx) = Fixture::load(&format_path("length_14_close_close_close.csv"));
        _test_close_close_close(&mut Stoch::new(ctx.clone(), 14), &df.test_target());
    }
}
