#[cfg(test)]
mod tests {
    use crate::{
        base::components::{
            component_context::ComponentContext,
            testing::{ComponentTestSnapshot, Fixture},
        },
        content::relative_vigor_index_indicator::{
            RelativeVigorIndexIndicator, RelativeVigorIndexIndicatorConfig,
        },
        utils::polars::DataFrameUtils,
    };

    fn format_path(path: &str) -> String {
        format!(
            "content/tests/fixtures/relative_vigor_index/indicator/{}",
            path
        )
    }

    fn _test(
        cctx: &mut ComponentContext,
        target: &mut RelativeVigorIndexIndicator,
        expected: &[Option<(Option<f64>, Option<f64>)>],
    ) {
        let mut snapshot = ComponentTestSnapshot::<(Option<f64>, Option<f64>)>::new();
        for cctx in cctx {
            let output = target.next();
            snapshot.push(Some((output.rvi, output.sig)));
        }
        snapshot.assert(expected);
    }

    #[test]
    fn length_14() {
        let (_df, ctx) = Fixture::raw(&format_path("length_14.csv"));
        let expected = _df.merge_two_columns("_target_rvi_", "_target_sig_");
        _test(
            &mut ctx.clone(),
            &mut RelativeVigorIndexIndicator::new(
                ctx.clone(),
                RelativeVigorIndexIndicatorConfig { length: 14 },
            ),
            &expected,
        );
    }

    #[test]
    fn length_1() {
        let (_df, ctx) = Fixture::raw(&format_path("length_1.csv"));
        let expected = _df.merge_two_columns("_target_rvi_", "_target_sig_");
        _test(
            &mut ctx.clone(),
            &mut RelativeVigorIndexIndicator::new(
                ctx.clone(),
                RelativeVigorIndexIndicatorConfig { length: 1 },
            ),
            &expected,
        );
    }

    #[test]
    fn length_2() {
        let (_df, ctx) = Fixture::raw(&format_path("length_2.csv"));
        let expected = _df.merge_two_columns("_target_rvi_", "_target_sig_");
        _test(
            &mut ctx.clone(),
            &mut RelativeVigorIndexIndicator::new(
                ctx.clone(),
                RelativeVigorIndexIndicatorConfig { length: 2 },
            ),
            &expected,
        );
    }

    #[test]
    fn length_3() {
        let (_df, ctx) = Fixture::raw(&format_path("length_3.csv"));
        let expected = _df.merge_two_columns("_target_rvi_", "_target_sig_");
        _test(
            &mut ctx.clone(),
            &mut RelativeVigorIndexIndicator::new(
                ctx.clone(),
                RelativeVigorIndexIndicatorConfig { length: 3 },
            ),
            &expected,
        );
    }

    #[test]
    fn length_365() {
        let (_df, ctx) = Fixture::raw(&format_path("length_365.csv"));
        let expected = _df.merge_two_columns("_target_rvi_", "_target_sig_");
        _test(
            &mut ctx.clone(),
            &mut RelativeVigorIndexIndicator::new(
                ctx.clone(),
                RelativeVigorIndexIndicatorConfig { length: 365 },
            ),
            &expected,
        );
    }
}
