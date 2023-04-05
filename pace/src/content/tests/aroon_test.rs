#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{
        content::aroon::{Aroon, AroonConfig},
        core::incremental::Incremental,
        polars::dataframe::DataFrameUtils,
        ta::simple_moving_average::Sma,
        testing::{
            array_snapshot::ArraySnapshot,
            fixture::{DataFrameFixtureUtils, Fixture},
            pace::format_pace_fixture_path,
        },
    };

    fn format_path(path: &str) -> PathBuf {
        format_pace_fixture_path(&format!("tests/content/aroon/indicator/{}", path))
    }

    fn _test(target: &mut Aroon, expected: &[(f64, f64)]) {
        let mut snapshot = ArraySnapshot::<(f64, f64)>::new();
        for _ in target.ctx.clone() {
            let output = target.next(());
            snapshot.push((output.up, output.down));
        }
        snapshot.assert(expected);
    }

    #[test]
    fn length_14() {
        let (df, ctx) = Fixture::load(&format_path("length_14.csv"));
        let expected = df.merge_two_columns("_target_up_", "_target_down_");
        _test(
            &mut Aroon::new(ctx.clone(), AroonConfig { length: 14 }),
            &expected,
        );
    }
}
