#[cfg(test)]
mod tests {
    use crate::{
        components::component::Component,
        content::balance_of_power_indicator::BopIndicator,
        testing::{
            array_snapshot::ArraySnapshot,
            fixture::{DataFrameFixtureUtils, Fixture},
        },
    };

    fn format_path(path: &str) -> String {
        format!("tests/content/balance_of_power/indicator/{}", path)
    }

    fn _test(target: &mut BopIndicator, expected: &[Option<f64>]) {
        let mut snapshot = ArraySnapshot::<Option<f64>>::new();
        for _ in target.ctx.clone() {
            let output = target.next(());
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn default() {
        let (df, ctx) = Fixture::load_ctx(&format_path("default.csv"));
        _test(&mut BopIndicator::new(ctx.clone()), &df.test_target());
    }
}
