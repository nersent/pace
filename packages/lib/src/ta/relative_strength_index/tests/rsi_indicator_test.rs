#[cfg(test)]
mod tests {
    use crate::{
        components::{
            component_context::ComponentContext,
            source::{Source, SourceKind},
            testing::ComponentTestSnapshot,
        },
        ta::relative_strength_index::rsi_indicator::{
            RelativeStrengthIndexIndicator, RelativeStrengthIndexIndicatorConfig,
        },
        testing::fixture::Fixture,
    };

    fn _test(
        cctx: &mut ComponentContext,
        target: &mut RelativeStrengthIndexIndicator,
        expected: &[Option<f64>],
    ) {
        let mut snapshot = ComponentTestSnapshot::<f64>::new();
        for cctx in cctx {
            let output = target.next();
            snapshot.push(output.rsi);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn test_rsi_btc_1d_length_14_open() {
        let (_df, ctx, expected) = Fixture::load(
            "ta/relative_strength_index/tests/fixtures/rsi_indicator/btc_1d_length_14_open.csv",
        );
        _test(
            &mut ctx.clone(),
            &mut RelativeStrengthIndexIndicator::new(
                ctx.clone(),
                RelativeStrengthIndexIndicatorConfig {
                    length: 14,
                    src: Source::from_kind(ctx.clone(), SourceKind::Open),
                },
            ),
            &expected,
        );
    }
}
