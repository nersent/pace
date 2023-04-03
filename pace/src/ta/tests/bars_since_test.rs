#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{
        core::incremental::Incremental,
        polars::series::SeriesCastUtils,
        ta::{
            average_true_range::Atr, bars_since::BarsSince, change::Change, dev::Dev,
            exponential_moving_average::Ema,
        },
        testing::{
            array_snapshot::ArraySnapshot,
            fixture::{DataFrameFixtureUtils, Fixture},
            pace::format_pace_fixture_path,
        },
    };

    fn format_path(path: &str) -> PathBuf {
        format_pace_fixture_path(&format!("tests/ta/bars_since/{}", path))
    }

    fn _test(target: &mut BarsSince, expected: &[Option<f64>], condition: &[Option<f64>]) {
        let mut snapshot = ArraySnapshot::<Option<f64>>::new();
        for i in target.ctx.clone() {
            let _condition = match condition[i] {
                Some(x) => x != 0.0,
                _ => false,
            };
            let output = target.next(_condition);
            snapshot.push(output.map(|x| x as f64));
        }
        snapshot.assert(expected);
    }

    #[test]
    fn a() {
        let (df, ctx) = Fixture::load_ctx(&format_path("a.csv"));
        _test(
            &mut BarsSince::new(ctx.clone()),
            &df.test_target(),
            &df.column("_target_condition_").unwrap().to_f64(),
        );
    }

    #[test]
    fn b() {
        let (df, ctx) = Fixture::load_ctx(&format_path("b.csv"));
        _test(
            &mut BarsSince::new(ctx.clone()),
            &df.test_target(),
            &df.column("_target_condition_").unwrap().to_f64(),
        );
    }

    #[test]
    fn c() {
        let (df, ctx) = Fixture::load_ctx(&format_path("c.csv"));
        _test(
            &mut BarsSince::new(ctx.clone()),
            &df.test_target(),
            &df.column("_target_condition_").unwrap().to_f64(),
        );
    }

    #[test]
    fn d() {
        let (df, ctx) = Fixture::load_ctx(&format_path("d.csv"));
        _test(
            &mut BarsSince::new(ctx.clone()),
            &df.test_target(),
            &df.column("_target_condition_").unwrap().to_f64(),
        );
    }
}
