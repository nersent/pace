#[cfg(test)]
mod tests {
    use crate::{
        components::{component::Component, src_component::SrcComponent, src_kind::SrcKind},
        content::stoch_relative_strength_index_indicator::{SrsiIndicator, SrsiIndicatorConfig},
        testing::{array_snapshot::ArraySnapshot, fixture::Fixture},
        utils::polars::DataFrameUtils,
    };

    fn format_path(path: &str) -> String {
        format!(
            "tests/content/stoch_relative_strength_index/indicator/{}",
            path
        )
    }

    fn _test(target: &mut SrsiIndicator, expected: &[Option<(Option<f64>, Option<f64>)>]) {
        let mut snapshot = ArraySnapshot::<Option<(Option<f64>, Option<f64>)>>::new();
        for _ in target.ctx.clone() {
            let output = target.next(());
            snapshot.push(Some((output.k, output.d)));
        }
        snapshot.assert(expected);
    }

    #[test]
    fn length_14_stoch_length_14_k_3_d_3_close() {
        let (df, ctx) =
            Fixture::load_ctx(&format_path("length_14_stoch_length_14_k_3_d_3_close.csv"));
        let expected = df.merge_two_columns("_target_k_", "_target_d_");
        _test(
            &mut SrsiIndicator::new(
                ctx.clone(),
                SrsiIndicatorConfig {
                    length_rsi: 14,
                    length_stoch: 14,
                    smooth_d: 3,
                    smooth_k: 3,
                    src: SrcComponent::build(ctx.clone(), SrcKind::Close),
                },
            ),
            &expected,
        );
    }

    #[test]
    fn length_2_stoch_length_2_k_3_d_3_close() {
        let (df, ctx) =
            Fixture::load_ctx(&format_path("length_2_stoch_length_2_k_3_d_3_close.csv"));
        let expected = df.merge_two_columns("_target_k_", "_target_d_");
        _test(
            &mut SrsiIndicator::new(
                ctx.clone(),
                SrsiIndicatorConfig {
                    length_rsi: 2,
                    length_stoch: 2,
                    smooth_d: 3,
                    smooth_k: 3,
                    src: SrcComponent::build(ctx.clone(), SrcKind::Close),
                },
            ),
            &expected,
        );
    }

    #[test]
    fn length_2_stoch_length_2_k_14_d_14_close() {
        let (df, ctx) =
            Fixture::load_ctx(&format_path("length_2_stoch_length_2_k_14_d_14_close.csv"));
        let expected = df.merge_two_columns("_target_k_", "_target_d_");
        _test(
            &mut SrsiIndicator::new(
                ctx.clone(),
                SrsiIndicatorConfig {
                    length_rsi: 2,
                    length_stoch: 2,
                    smooth_d: 14,
                    smooth_k: 14,
                    src: SrcComponent::build(ctx.clone(), SrcKind::Close),
                },
            ),
            &expected,
        );
    }
}
