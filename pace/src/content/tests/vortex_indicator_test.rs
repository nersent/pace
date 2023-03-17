#[cfg(test)]
mod tests {
    use crate::{
        components::component::Component,
        content::vortex_indicator::{VortexIndicator, VortexIndicatorConfig},
        testing::{
            array_snapshot::ArraySnapshot,
            fixture::{DataFrameFixtureUtils, Fixture},
        },
        utils::polars::DataFrameUtils,
    };

    fn format_path(path: &str) -> String {
        format!("tests/content/vortex/indicator/{}", path)
    }

    fn _test(target: &mut VortexIndicator, expected: &[Option<(Option<f64>, Option<f64>)>]) {
        let mut snapshot = ArraySnapshot::<Option<(Option<f64>, Option<f64>)>>::new();
        for _ in target.ctx.clone() {
            let output = target.next(());
            snapshot.push(Some((output.plus, output.minus)));
        }
        snapshot.assert(expected);
    }

    #[test]
    fn length_14() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_14.csv"));
        let expected = df.merge_two_columns("_target_plus_", "_target_minus_");
        _test(
            &mut VortexIndicator::new(ctx.clone(), VortexIndicatorConfig { length: 14 }),
            &expected,
        );
    }

    #[test]
    fn length_2() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_2.csv"));
        let expected = df.merge_two_columns("_target_plus_", "_target_minus_");
        _test(
            &mut VortexIndicator::new(ctx.clone(), VortexIndicatorConfig { length: 2 }),
            &expected,
        );
    }
}
