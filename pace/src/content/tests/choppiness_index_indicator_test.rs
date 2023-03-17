#[cfg(test)]
mod tests {
    use crate::{
        components::component::Component,
        content::choppiness_index_indicator::{CiIndicator, CiIndicatorConfig},
        testing::{
            array_snapshot::ArraySnapshot,
            fixture::{DataFrameFixtureUtils, Fixture},
        },
    };

    fn format_path(path: &str) -> String {
        format!("tests/content/choppiness_index/indicator/{}", path)
    }

    fn _test(target: &mut CiIndicator, expected: &[Option<f64>]) {
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
            &mut CiIndicator::new(ctx.clone(), CiIndicatorConfig { length: 14 }),
            &df.test_target(),
        );
    }

    #[test]
    fn length_2() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_2.csv"));
        _test(
            &mut CiIndicator::new(ctx.clone(), CiIndicatorConfig { length: 2 }),
            &df.test_target(),
        );
    }
}
