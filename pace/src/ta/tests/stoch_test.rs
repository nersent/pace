#[cfg(test)]
mod tests {
    use crate::{
        core::incremental::Incremental,
        ta::{
            average_true_range::Atr, change::Change, exponential_moving_average::Ema, stoch::Stoch,
        },
        testing::{
            array_snapshot::ArraySnapshot,
            fixture::{DataFrameFixtureUtils, Fixture},
        },
    };

    fn format_path(path: &str) -> String {
        format!("tests/ta/stoch/{}", path)
    }

    fn _test_close_high_low(target: &mut Stoch, expected: &[Option<f64>]) {
        let mut snapshot = ArraySnapshot::<Option<f64>>::new();
        for _ in target.ctx.clone() {
            let bar = target.ctx.bar();
            let ouptut = target.next((bar.close, bar.high, bar.low));
            snapshot.push(ouptut);
        }
        snapshot.assert(expected);
    }

    fn _test_close_close_close(target: &mut Stoch, expected: &[Option<f64>]) {
        let mut snapshot = ArraySnapshot::<Option<f64>>::new();
        for _ in target.ctx.clone() {
            let bar = target.ctx.bar();
            let ouptut = target.next((bar.close, bar.close, bar.close));
            snapshot.push(ouptut);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn length_14_close_high_low() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_14_close_high_low.csv"));
        _test_close_high_low(&mut Stoch::new(ctx.clone(), 14), &df.test_target());
    }

    #[test]
    fn length_1_close_high_low() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_1_close_high_low.csv"));
        _test_close_high_low(&mut Stoch::new(ctx.clone(), 1), &df.test_target());
    }

    #[test]
    fn length_2_close_high_low() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_2_close_high_low.csv"));
        _test_close_high_low(&mut Stoch::new(ctx.clone(), 2), &df.test_target());
    }

    #[test]
    fn length_3_close_high_low() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_3_close_high_low.csv"));
        _test_close_high_low(&mut Stoch::new(ctx.clone(), 3), &df.test_target());
    }

    #[test]
    fn length_1_close_close_close() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_1_close_close_close.csv"));
        _test_close_close_close(&mut Stoch::new(ctx.clone(), 1), &df.test_target());
    }

    #[test]
    fn length_2_close_close_close() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_2_close_close_close.csv"));
        _test_close_close_close(&mut Stoch::new(ctx.clone(), 2), &df.test_target());
    }

    #[test]
    fn length_3_close_close_close() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_3_close_close_close.csv"));
        _test_close_close_close(&mut Stoch::new(ctx.clone(), 3), &df.test_target());
    }

    #[test]
    fn length_14_close_close_close() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_14_close_close_close.csv"));
        _test_close_close_close(&mut Stoch::new(ctx.clone(), 14), &df.test_target());
    }
}
