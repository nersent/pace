#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{
        core::incremental::Incremental,
        ta::{
            average_true_range::Atr, change::Change, cross::CrossMode,
            cross_threshold::CrossThreshold, exponential_moving_average::Ema,
            relative_strength_index::Rsi,
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

    fn _test(target: &mut CrossThreshold, target_rsi: &mut Rsi, mode: CrossMode, expected: &[f64]) {
        let mut snapshot = ArraySnapshot::<f64>::new();
        for _ in target.ctx.clone() {
            let output_rsi = target_rsi.next(target.ctx.bar.close());
            let output = target.next(output_rsi);
            let output = match output {
                Some(output) => output == mode,
                None => false,
            };
            let output = if output { 1.0 } else { 0.0 };
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn over_with_rsi_length_14_close() {
        let (df, ctx) = Fixture::load(&format_path("over/rsi/length_14_close.csv"));
        _test(
            &mut CrossThreshold::new(ctx.clone(), 30.0),
            &mut Rsi::new(ctx.clone(), 14),
            CrossMode::Over,
            &df.test_target(),
        );
    }

    #[test]
    fn under_with_rsi_length_14_close() {
        let (df, ctx) = Fixture::load(&format_path("under/rsi/length_14_close.csv"));
        _test(
            &mut CrossThreshold::new(ctx.clone(), 70.0),
            &mut Rsi::new(ctx.clone(), 14),
            CrossMode::Under,
            &df.test_target(),
        );
    }
}
