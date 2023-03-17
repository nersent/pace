#[cfg(test)]
mod tests {
    use crate::{
        components::component::Component,
        content::chaikin_money_flow_indicator::{CmfIndicator, CmfIndicatorConfig},
        testing::{
            array_snapshot::ArraySnapshot,
            fixture::{DataFrameFixtureUtils, Fixture},
        },
    };

    fn format_path(path: &str) -> String {
        format!("tests/content/chaikin_money_flow/indicator/{}", path)
    }

    fn _test(target: &mut CmfIndicator, expected: &[Option<f64>]) {
        let mut snapshot = ArraySnapshot::<Option<f64>>::new();
        for _ in target.ctx.clone() {
            let output = target.next(());
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn length_14() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_14.csv"));
        _test(
            &mut CmfIndicator::new(ctx.clone(), CmfIndicatorConfig { length: 14 }),
            &df.test_target(),
        );
    }
}
