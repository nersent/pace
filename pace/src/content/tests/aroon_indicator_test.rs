#[cfg(test)]
mod tests {
    use crate::{
        components::component::Component,
        content::aroon_indicator::{AroonIndicator, AroonIndicatorConfig},
        testing::{array_snapshot::ArraySnapshot, fixture::Fixture},
        utils::polars::DataFrameUtils,
    };

    fn format_path(path: &str) -> String {
        format!("tests/content/aroon/indicator/{}", path)
    }

    fn _test(target: &mut AroonIndicator, expected: &[Option<(Option<f64>, Option<f64>)>]) {
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
            &mut AroonIndicator::new(ctx.clone(), AroonIndicatorConfig { length: 14 }),
            &expected,
        );
    }
}
