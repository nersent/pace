#[cfg(test)]
mod tests {
    use crate::{
        content::aroon::{Aroon, AroonConfig},
        core::incremental::Incremental,
        polars::dataframe::DataFrameUtils,
        ta::simple_moving_average::Sma,
        testing::{
            array_snapshot::ArraySnapshot,
            fixture::{DataFrameFixtureUtils, Fixture},
        },
    };

    fn format_path(path: &str) -> String {
        format!("tests/content/aroon/indicator/{}", path)
    }

    fn _test(target: &mut Aroon, expected: &[Option<(Option<f64>, Option<f64>)>]) {
        let mut snapshot = ArraySnapshot::<Option<(Option<f64>, Option<f64>)>>::new();
        for _ in target.ctx.clone() {
            let output = target.next(());
            snapshot.push(Some((output.up, output.down)));
        }
        snapshot.assert(expected);
    }

    #[test]
    fn length_14() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_14.csv"));
        let expected = df.merge_two_columns("_target_up_", "_target_down_");
        _test(
            &mut Aroon::new(ctx.clone(), AroonConfig { length: 14 }),
            &expected,
        );
    }
}
