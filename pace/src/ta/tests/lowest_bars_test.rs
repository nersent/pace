#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{
        core::incremental::Incremental,
        polars::series::SeriesCastUtils,
        ta::{
            average_true_range::Atr, change::Change, exponential_moving_average::Ema,
            lowest_bars::LowestBars,
        },
        testing::{
            array_snapshot::ArraySnapshot,
            fixture::{DataFrameFixtureUtils, Fixture},
            pace::format_pace_fixture_path,
        },
        utils::float::OptionFloatUtils,
    };

    fn format_path(path: &str) -> PathBuf {
        format_pace_fixture_path(&format!("tests/ta/bars/lowest_bars/{}", path))
    }

    fn _test_with_src(target: &mut LowestBars, src: &[f64], expected: &[f64]) {
        let mut snapshot = ArraySnapshot::<f64>::new();
        for i in target.ctx.clone() {
            let output = target.next(src[i]);
            snapshot.push(output.unwrap_nan());
        }
        snapshot.assert(expected);
    }

    #[test]
    fn length_14_low() {
        let (df, ctx) = Fixture::load(&format_path("length_14_low.csv"));
        _test_with_src(
            &mut LowestBars::new(ctx.clone(), 14),
            &df.column("_target_src_").unwrap().to_f64(),
            &df.test_target(),
        );
    }

    #[test]
    fn length_14_high() {
        let (df, ctx) = Fixture::load(&format_path("length_14_high.csv"));
        _test_with_src(
            &mut LowestBars::new(ctx.clone(), 14),
            &df.column("_target_src_").unwrap().to_f64(),
            &df.test_target(),
        );
    }
}
