#[cfg(test)]
mod tests {
    use crate::{
        components::component::Component,
        content::relative_vigor_index_indicator::{RvgiIndicator, RvgiIndicatorConfig},
        testing::{array_snapshot::ArraySnapshot, fixture::Fixture},
        utils::polars::DataFrameUtils,
    };

    fn format_path(path: &str) -> String {
        format!("tests/content/relative_vigor_index/indicator/{}", path)
    }

    fn _test(target: &mut RvgiIndicator, expected: &[Option<(Option<f64>, Option<f64>)>]) {
        let mut snapshot = ArraySnapshot::<Option<(Option<f64>, Option<f64>)>>::new();
        for _ in target.ctx.clone() {
            let output = target.next(());
            snapshot.push(Some((output.rvi, output.sig)));
        }
        snapshot.assert(expected);
    }

    #[test]
    fn length_14() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_14.csv"));
        let expected = df.merge_two_columns("_target_rvi_", "_target_sig_");
        _test(
            &mut RvgiIndicator::new(ctx.clone(), RvgiIndicatorConfig { length: 14 }),
            &expected,
        );
    }

    #[test]
    fn length_1() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_1.csv"));
        let expected = df.merge_two_columns("_target_rvi_", "_target_sig_");
        _test(
            &mut RvgiIndicator::new(ctx.clone(), RvgiIndicatorConfig { length: 1 }),
            &expected,
        );
    }

    #[test]
    fn length_2() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_2.csv"));
        let expected = df.merge_two_columns("_target_rvi_", "_target_sig_");
        _test(
            &mut RvgiIndicator::new(ctx.clone(), RvgiIndicatorConfig { length: 2 }),
            &expected,
        );
    }

    #[test]
    fn length_3() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_3.csv"));
        let expected = df.merge_two_columns("_target_rvi_", "_target_sig_");
        _test(
            &mut RvgiIndicator::new(ctx.clone(), RvgiIndicatorConfig { length: 3 }),
            &expected,
        );
    }

    #[test]
    fn length_365() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_365.csv"));
        let expected = df.merge_two_columns("_target_rvi_", "_target_sig_");
        _test(
            &mut RvgiIndicator::new(ctx.clone(), RvgiIndicatorConfig { length: 365 }),
            &expected,
        );
    }
}
