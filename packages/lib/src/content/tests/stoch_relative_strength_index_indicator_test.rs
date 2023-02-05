#[cfg(test)]
mod tests {
    use crate::{
        base::{
            asset::source::{Source, SourceKind},
            components::{
                component_context::ComponentContext,
                testing::{ComponentTestSnapshot, Fixture},
            },
        },
        content::stoch_relative_volatility_index_indicator::{
            StochRelativeStrengthIndexIndicator, StochRelativeStrengthIndexIndicatorConfig,
        },
        utils::polars::DataFrameUtils,
    };

    fn format_path(path: &str) -> String {
        format!(
            "content/tests/fixtures/stoch_relative_strength_index/indicator/{}",
            path
        )
    }

    fn _test(
        cctx: &mut ComponentContext,
        target: &mut StochRelativeStrengthIndexIndicator,
        expected: &[Option<(Option<f64>, Option<f64>)>],
    ) {
        let mut snapshot = ComponentTestSnapshot::<(Option<f64>, Option<f64>)>::new();
        for cctx in cctx {
            let output = target.next();
            snapshot.push(Some((output.k, output.d)));
        }
        snapshot.assert(expected);
    }

    #[test]
    fn length_14_stoch_length_14_k_3_d_3_close() {
        let (_df, ctx) = Fixture::raw(&format_path("length_14_stoch_length_14_k_3_d_3_close.csv"));
        let expected = _df.merge_two_columns("_target_k_", "_target_d_");
        _test(
            &mut ctx.clone(),
            &mut StochRelativeStrengthIndexIndicator::new(
                ctx.clone(),
                StochRelativeStrengthIndexIndicatorConfig {
                    length_rsi: 14,
                    length_stoch: 14,
                    smooth_d: 3,
                    smooth_k: 3,
                    src: Source::from_kind(ctx.clone(), SourceKind::Close),
                },
            ),
            &expected,
        );
    }

    #[test]
    fn length_2_stoch_length_2_k_3_d_3_close() {
        let (_df, ctx) = Fixture::raw(&format_path("length_2_stoch_length_2_k_3_d_3_close.csv"));
        let expected = _df.merge_two_columns("_target_k_", "_target_d_");
        _test(
            &mut ctx.clone(),
            &mut StochRelativeStrengthIndexIndicator::new(
                ctx.clone(),
                StochRelativeStrengthIndexIndicatorConfig {
                    length_rsi: 2,
                    length_stoch: 2,
                    smooth_d: 3,
                    smooth_k: 3,
                    src: Source::from_kind(ctx.clone(), SourceKind::Close),
                },
            ),
            &expected,
        );
    }

    #[test]
    fn length_2_stoch_length_2_k_14_d_14_close() {
        let (_df, ctx) = Fixture::raw(&format_path("length_2_stoch_length_2_k_14_d_14_close.csv"));
        let expected = _df.merge_two_columns("_target_k_", "_target_d_");
        _test(
            &mut ctx.clone(),
            &mut StochRelativeStrengthIndexIndicator::new(
                ctx.clone(),
                StochRelativeStrengthIndexIndicatorConfig {
                    length_rsi: 2,
                    length_stoch: 2,
                    smooth_d: 14,
                    smooth_k: 14,
                    src: Source::from_kind(ctx.clone(), SourceKind::Close),
                },
            ),
            &expected,
        );
    }
}
