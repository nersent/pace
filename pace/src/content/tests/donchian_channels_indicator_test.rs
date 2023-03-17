#[cfg(test)]
mod tests {
    use crate::{
        components::component::Component,
        content::donchian_channels_indicator::{DcIndicator, DcIndicatorConfig},
        testing::{array_snapshot::ArraySnapshot, fixture::Fixture},
        utils::polars::DataFrameUtils,
    };

    fn format_path(path: &str) -> String {
        format!("tests/content/donchian_channels/indicator/{}", path)
    }

    fn _test(
        target: &mut DcIndicator,
        expected: &[Option<(Option<f64>, Option<f64>, Option<f64>)>],
    ) {
        let mut snapshot = ArraySnapshot::<Option<(Option<f64>, Option<f64>, Option<f64>)>>::new();
        for _ in target.ctx.clone() {
            let output = target.next(());
            snapshot.push(Some((output.upper, output.basis, output.lower)));
        }
        snapshot.assert(expected);
    }

    #[test]
    fn length_14() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_14.csv"));
        let expected = df.merge_three_columns("_target_upper_", "_target_basis_", "_target_lower_");
        _test(
            &mut DcIndicator::new(ctx.clone(), DcIndicatorConfig { length: 14 }),
            &expected,
        );
    }
}
