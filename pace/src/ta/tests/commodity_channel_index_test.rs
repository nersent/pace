#[cfg(test)]
#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{
        core::incremental::Incremental,
        ta::{
            average_true_range::Atr, commodity_channel_index::Cci, exponential_moving_average::Ema,
        },
        testing::{
            array_snapshot::ArraySnapshot,
            fixture::{DataFrameFixtureUtils, Fixture},
            pace::format_pace_fixture_path,
        },
    };

    fn format_path(path: &str) -> PathBuf {
        format_pace_fixture_path(&format!("tests/ta/cci/{}", path))
    }

    fn _test(target: &mut Cci, expected: &[f64]) {
        let mut snapshot = ArraySnapshot::<f64>::new();
        for _ in target.ctx.clone() {
            let src = target.ctx.bar.close();
            let output = target.next(src);
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    // @TODO: TradingView CCI indicator has different implementation that `ta.cci` for length < 4

    // #[test]
    // fn length_1() {
    //     let (df, ctx) = Fixture::load(&format_path("length_1.csv"));
    //     _test(&mut Cci::new(ctx.clone(), 1), &df.test_target());
    // }

    // #[test]
    // fn length_2() {
    //     let (df, ctx) = Fixture::load(&format_path("length_2.csv"));
    //     _test(&mut Cci::new(ctx.clone(), 2), &df.test_target());
    // }

    // #[test]
    // fn length_3() {
    //     let (df, ctx) = Fixture::load(&format_path("length_3.csv"));
    //     _test(&mut Cci::new(ctx.clone(), 3), &df.test_target());
    // }

    #[test]
    fn length_5() {
        let (df, ctx) = Fixture::load(&format_path("length_5.csv"));
        _test(&mut Cci::new(ctx.clone(), 5), &df.test_target());
    }

    #[test]
    fn length_14() {
        let (df, ctx) = Fixture::load(&format_path("length_14.csv"));
        _test(&mut Cci::new(ctx.clone(), 14), &df.test_target());
    }

    #[test]
    fn length_365() {
        let (df, ctx) = Fixture::load(&format_path("length_365.csv"));
        _test(&mut Cci::new(ctx.clone(), 365), &df.test_target());
    }
}
