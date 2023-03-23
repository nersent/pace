#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{
        core::incremental::Incremental,
        ta::{
            average_true_range::Atr, change::Change, cross_under::CrossUnder,
            exponential_moving_average::Ema, relative_strength_index::Rsi,
        },
        testing::{
            array_snapshot::ArraySnapshot,
            fixture::{DataFrameFixtureUtils, Fixture},
            pace::format_pace_fixture_path,
        },
    };

    fn format_path(path: &str) -> PathBuf {
        format_pace_fixture_path(&format!("tests/ta/cross/{}", path))
    }

    fn _test(
        target: &mut CrossUnder,
        target_rsi: &mut Rsi,
        threshold: Option<f64>,
        expected: &[Option<f64>],
    ) {
        let mut snapshot = ArraySnapshot::<Option<f64>>::new();
        for _ in target.ctx.clone() {
            let output_rsi = target_rsi.next(target.ctx.bar.close());
            let output = target.next((output_rsi, threshold));
            snapshot.push(Some(if output { 1.0 } else { 0.0 }));
        }
        snapshot.assert(expected);
    }

    #[test]
    fn under_with_rsi_length_14_close() {
        let (df, ctx) = Fixture::load_ctx(&format_path("under/rsi/length_14_close.csv"));
        _test(
            &mut CrossUnder::new(ctx.clone()),
            &mut Rsi::new(ctx.clone(), 14),
            Some(70.0),
            &df.test_target(),
        );
    }
}
