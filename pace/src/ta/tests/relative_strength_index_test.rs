#[cfg(test)]
mod tests {

    use std::path::PathBuf;

    use crate::{
        core::incremental::Incremental,
        ta::{
            average_true_range::Atr, change::Change, exponential_moving_average::Ema,
            relative_strength_index::Rsi, simple_moving_average::Sma,
        },
        testing::{
            array_snapshot::ArraySnapshot,
            fixture::{DataFrameFixtureUtils, Fixture},
            pace::format_pace_fixture_path,
        },
    };

    fn format_path(path: &str) -> PathBuf {
        format_pace_fixture_path(&format!("tests/ta/rsi/{}", path))
    }

    fn _test(target: &mut Rsi, expected: &[f64]) {
        let mut snapshot = ArraySnapshot::<f64>::new();
        for _ in target.ctx.clone() {
            let output = target.next(target.ctx.bar.close());
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    // #[test]
    // fn length_2_close() {
    //     let (df, ctx) = Fixture::load(&format_path("length_2_close.csv"));
    //     _test(&mut Rsi::new(ctx.clone(), 2), &df.test_target());
    // }

    // #[test]
    // fn length_3_close() {
    //     let (df, ctx) = Fixture::load(&format_path("length_3_close.csv"));
    //     _test(&mut Rsi::new(ctx.clone(), 3), &df.test_target());
    // }

    // #[test]
    // fn length_7_close() {
    //     let (df, ctx) = Fixture::load(&format_path("length_7_close.csv"));
    //     _test(&mut Rsi::new(ctx.clone(), 7), &df.test_target());
    // }

    // #[test]
    // fn length_14_close() {
    //     let (df, ctx) = Fixture::load(&format_path("length_14_close.csv"));
    //     _test(&mut Rsi::new(ctx.clone(), 14), &df.test_target());
    // }

    // #[test]
    // fn length_350_close() {
    //     let (df, ctx) = Fixture::load(&format_path("length_350_close.csv"));
    //     _test(&mut Rsi::new(ctx.clone(), 350), &df.test_target());
    // }

    // fn _test_with_sma(target: &mut Rsi, target_sma: &mut Sma, expected: &[f64]) {
    //     let mut snapshot = ArraySnapshot::<f64>::new();
    //     for _ in target.ctx.clone() {
    //         let output_sma = target_sma.next(target.ctx.bar.close());
    //         let output_rsi = target.next(output_sma);
    //         snapshot.push(output_rsi);
    //     }
    //     snapshot.assert(expected);
    // }

    // #[test]
    // fn length_2_with_sma_length_14_close() {
    //     let (df, ctx) =
    //         Fixture::load(&format_path("sma/length_2_with_sma_length_14_close.csv"));
    //     _test_with_sma(
    //         &mut Rsi::new(ctx.clone(), 2),
    //         &mut Sma::new(ctx.clone(), 14),
    //         &df.test_target(),
    //     );
    // }

    // #[test]
    // fn length_14_with_sma_length_2_close() {
    //     let (df, ctx) =
    //         Fixture::load(&format_path("sma/length_14_with_sma_length_2_close.csv"));
    //     _test_with_sma(
    //         &mut Rsi::new(ctx.clone(), 14),
    //         &mut Sma::new(ctx.clone(), 2),
    //         &df.test_target(),
    //     );
    // }

    // #[test]
    // fn length_14_with_sma_length_14_close() {
    //     let (df, ctx) =
    //         Fixture::load(&format_path("sma/length_14_with_sma_length_14_close.csv"));
    //     _test_with_sma(
    //         &mut Rsi::new(ctx.clone(), 14),
    //         &mut Sma::new(ctx.clone(), 14),
    //         &df.test_target(),
    //     );
    // }
}
