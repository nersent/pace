#[cfg(test)]
mod tests {
    use crate::{
        components::component::Component,
        content::directional_movement_index_indicator::{DmiIndicator, DmiIndicatorConfig},
        testing::{array_snapshot::ArraySnapshot, fixture::Fixture},
        utils::polars::DataFrameUtils,
    };

    fn format_path(path: &str) -> String {
        format!(
            "tests/content/directional_movement_index/indicator/{}",
            path
        )
    }

    fn _test(
        target: &mut DmiIndicator,
        expected: &[Option<(Option<f64>, Option<f64>, Option<f64>)>],
    ) {
        let mut snapshot = ArraySnapshot::<Option<(Option<f64>, Option<f64>, Option<f64>)>>::new();
        for _ in target.ctx.clone() {
            let output = target.next(());
            snapshot.push(Some((output.plus, output.minus, output.adx)));
        }
        snapshot.assert(expected);
    }

    #[test]
    fn length_14_lensig_14() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_14_lensig_14.csv"));
        let expected = df.merge_three_columns("_target_plus_", "_target_minus_", "_target_adx_");
        _test(
            &mut DmiIndicator::new(
                ctx.clone(),
                DmiIndicatorConfig {
                    length: 14,
                    lensig: 14,
                },
            ),
            &expected,
        );
    }

    #[test]
    fn length_3_lensig_3() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_3_lensig_3.csv"));
        let expected = df.merge_three_columns("_target_plus_", "_target_minus_", "_target_adx_");
        _test(
            &mut DmiIndicator::new(
                ctx.clone(),
                DmiIndicatorConfig {
                    length: 3,
                    lensig: 3,
                },
            ),
            &expected,
        );
    }

    #[test]
    fn length_14_lensig_3() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_14_lensig_3.csv"));
        let expected = df.merge_three_columns("_target_plus_", "_target_minus_", "_target_adx_");
        _test(
            &mut DmiIndicator::new(
                ctx.clone(),
                DmiIndicatorConfig {
                    length: 14,
                    lensig: 3,
                },
            ),
            &expected,
        );
    }

    #[test]
    fn length_3_lensig_14() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_3_lensig_14.csv"));
        let expected = df.merge_three_columns("_target_plus_", "_target_minus_", "_target_adx_");
        _test(
            &mut DmiIndicator::new(
                ctx.clone(),
                DmiIndicatorConfig {
                    length: 3,
                    lensig: 14,
                },
            ),
            &expected,
        );
    }
}
