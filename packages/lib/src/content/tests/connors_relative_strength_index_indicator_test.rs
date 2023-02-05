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
        content::connors_relative_volatility_index_indicator::{
            ConnorsRelativeStrengthIndexIndicator, ConnorsRelativeStrengthIndexIndicatorConfig,
        },
    };

    fn format_path(path: &str) -> String {
        format!(
            "content/tests/fixtures/connors_relative_strength_index/indicator/{}",
            path
        )
    }

    fn _test(
        cctx: &mut ComponentContext,
        target: &mut ConnorsRelativeStrengthIndexIndicator,
        expected: &[Option<f64>],
    ) {
        let mut snapshot = ComponentTestSnapshot::<f64>::new();
        for cctx in cctx {
            let output = target.next();
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn length_3_up_down_len_2_roc_length_100_close() {
        let (_df, ctx, expected) = Fixture::load(&format_path(
            "length_3_up_down_len_2_roc_length_100_close.csv",
        ));
        _test(
            &mut ctx.clone(),
            &mut ConnorsRelativeStrengthIndexIndicator::new(
                ctx.clone(),
                ConnorsRelativeStrengthIndexIndicatorConfig {
                    length_rsi: 3,
                    length_up_down: 2,
                    length_roc: 100,
                    src: Source::from_kind(ctx.clone(), SourceKind::Close),
                },
            ),
            &expected,
        );
    }
}
