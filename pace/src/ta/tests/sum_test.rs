#[cfg(test)]
mod tests {

    use std::path::PathBuf;

    use crate::{
        core::incremental::Incremental,
        polars::series::SeriesCastUtils,
        ta::{average_true_range::Atr, change::Change, exponential_moving_average::Ema, sum::Sum},
        testing::{
            array_snapshot::ArraySnapshot,
            fixture::{DataFrameFixtureUtils, Fixture},
            pace::format_pace_fixture_path,
        },
    };

    fn format_path(path: &str) -> PathBuf {
        format_pace_fixture_path(&format!("tests/ta/sum/{}", path))
    }

    fn _test(target: &mut Sum, expected: &[f64]) {
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
        _test(&mut Sum::new(ctx.clone(), 1), &df.test_target());
    }

    #[test]
    fn length_2_close() {
        let (df, ctx) = Fixture::load(&format_path("length_2_close.csv"));
        _test(&mut Sum::new(ctx.clone(), 2), &df.test_target());
    }

    #[test]
    fn length_3_close() {
        let (df, ctx) = Fixture::load(&format_path("length_3_close.csv"));
        _test(&mut Sum::new(ctx.clone(), 3), &df.test_target());
    }

    #[test]
    fn length_14_close() {
        let (df, ctx) = Fixture::load(&format_path("length_14_close.csv"));
        _test(&mut Sum::new(ctx.clone(), 14), &df.test_target());
    }

    #[test]
    fn length_365_close() {
        let (df, ctx) = Fixture::load(&format_path("length_365_close.csv"));
        _test(&mut Sum::new(ctx.clone(), 365), &df.test_target());
    }

    // fn _test_with_atr(target: &mut Sum, target_atr: &mut Atr, expected: &[f64]) {
    //     let mut snapshot = ArraySnapshot::<f64>::new();
    //     for _ in target.ctx.clone() {
    //         let atr = target_atr.next(());
    //         let output = target.next(atr);
    //         snapshot.push(output);
    //     }
    //     snapshot.assert(expected);
    // }

    // #[test]
    // fn length_1_with_atr_length_14() {
    //     let (df, ctx) = Fixture::load(&format_path("atr/length_1_with_atr_length_14.csv"));
    //     _test_with_atr(
    //         &mut Sum::new(ctx.clone(), 1),
    //         &mut Atr::new(ctx.clone(), 14),
    //         &df.test_target(),
    //     );
    // }

    // #[test]
    // fn length_14_with_atr_length_14() {
    //     let (df, ctx) = Fixture::load(&format_path("atr/length_14_with_atr_length_14.csv"));
    //     _test_with_atr(
    //         &mut Sum::new(ctx.clone(), 14),
    //         &mut Atr::new(ctx.clone(), 14),
    //         &df.test_target(),
    //     );
    // }

    // #[test]
    // fn length_1_with_atr_length_1() {
    //     let (df, ctx) = Fixture::load(&format_path("atr/length_1_with_atr_length_1.csv"));
    //     _test_with_atr(
    //         &mut Sum::new(ctx.clone(), 1),
    //         &mut Atr::new(ctx.clone(), 1),
    //         &df.test_target(),
    //     );
    // }

    // #[test]
    // fn length_14_with_atr_length_1() {
    //     let (df, ctx) = Fixture::load(&format_path("atr/length_14_with_atr_length_1.csv"));
    //     _test_with_atr(
    //         &mut Sum::new(ctx.clone(), 14),
    //         &mut Atr::new(ctx.clone(), 1),
    //         &df.test_target(),
    //     );
    // }
}
