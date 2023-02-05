#[cfg(test)]
mod tests {
    use crate::{
        base::components::{
            component_context::ComponentContext,
            testing::{ComponentTestSnapshot, Fixture},
        },
        content::directional_movement_index_indicator::{
            DirectionalMovementIndexIndicator, DirectionalMovementIndexIndicatorConfig,
        },
        utils::polars::DataFrameUtils,
    };

    fn format_path(path: &str) -> String {
        format!(
            "content/tests/fixtures/directional_movement_index/indicator/{}",
            path
        )
    }

    fn _test(
        cctx: &mut ComponentContext,
        target: &mut DirectionalMovementIndexIndicator,
        expected: &[Option<(Option<f64>, Option<f64>, Option<f64>)>],
    ) {
        let mut snapshot = ComponentTestSnapshot::<(Option<f64>, Option<f64>, Option<f64>)>::new();
        for cctx in cctx {
            let output = target.next();
            snapshot.push(Some((output.plus, output.minus, output.adx)));
        }
        snapshot.assert(expected);
    }

    #[test]
    fn length_14_lensig_14() {
        let (_df, ctx) = Fixture::raw(&format_path("length_14_lensig_14.csv"));
        let expected = _df.merge_three_columns("_target_plus_", "_target_minus_", "_target_adx_");
        _test(
            &mut ctx.clone(),
            &mut DirectionalMovementIndexIndicator::new(
                ctx.clone(),
                DirectionalMovementIndexIndicatorConfig {
                    length: 14,
                    lensig: 14,
                },
            ),
            &expected,
        );
    }

    #[test]
    fn length_3_lensig_3() {
        let (_df, ctx) = Fixture::raw(&format_path("length_3_lensig_3.csv"));
        let expected = _df.merge_three_columns("_target_plus_", "_target_minus_", "_target_adx_");
        _test(
            &mut ctx.clone(),
            &mut DirectionalMovementIndexIndicator::new(
                ctx.clone(),
                DirectionalMovementIndexIndicatorConfig {
                    length: 3,
                    lensig: 3,
                },
            ),
            &expected,
        );
    }

    #[test]
    fn length_14_lensig_3() {
        let (_df, ctx) = Fixture::raw(&format_path("length_14_lensig_3.csv"));
        let expected = _df.merge_three_columns("_target_plus_", "_target_minus_", "_target_adx_");
        _test(
            &mut ctx.clone(),
            &mut DirectionalMovementIndexIndicator::new(
                ctx.clone(),
                DirectionalMovementIndexIndicatorConfig {
                    length: 14,
                    lensig: 3,
                },
            ),
            &expected,
        );
    }

    #[test]
    fn length_3_lensig_14() {
        let (_df, ctx) = Fixture::raw(&format_path("length_3_lensig_14.csv"));
        let expected = _df.merge_three_columns("_target_plus_", "_target_minus_", "_target_adx_");
        _test(
            &mut ctx.clone(),
            &mut DirectionalMovementIndexIndicator::new(
                ctx.clone(),
                DirectionalMovementIndexIndicatorConfig {
                    length: 3,
                    lensig: 14,
                },
            ),
            &expected,
        );
    }
}
