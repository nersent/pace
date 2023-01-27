#[cfg(test)]
mod tests {
    use crate::{
        components::{component_context::ComponentContext, testing::ComponentTestSnapshot},
        ta::{
            cross::{cross::CrossMode, cross_component::CrossComponent},
            relative_strength_index::rsi_component::RelativeStrengthIndexComponent,
        },
        testing::fixture::Fixture,
    };

    fn _test(
        cctx: &mut ComponentContext,
        target_cross: &mut CrossComponent,
        target_rsi: &mut RelativeStrengthIndexComponent,
        threshold: Option<f64>,
        expected: &[Option<f64>],
    ) {
        let mut snapshot = ComponentTestSnapshot::<f64>::new();
        for cctx in cctx {
            let output_rsi = target_rsi.next(cctx.get().close());
            let output = target_cross.next(output_rsi.rsi, threshold);
            snapshot.push(Some(if output { 1.0 } else { 0.0 }));
        }
        snapshot.assert(expected);
    }

    #[test]
    fn test_cross_over_with_rsi_btc_1d_length_14_close() {
        let (_df, ctx, expected) =
            Fixture::load("ta/cross/tests/fixtures/cross_over/rsi/btc_1d_length_14_close.csv");
        _test(
            &mut ctx.clone(),
            &mut CrossComponent::new(ctx.clone(), CrossMode::Over),
            &mut RelativeStrengthIndexComponent::new(ctx.clone(), 14),
            Some(30.0),
            &expected,
        );
    }

    #[test]
    fn test_cross_under_with_rsi_btc_1d_length_14_close() {
        let (_df, ctx, expected) =
            Fixture::load("ta/cross/tests/fixtures/cross_under/rsi/btc_1d_length_14_close.csv");
        _test(
            &mut ctx.clone(),
            &mut CrossComponent::new(ctx.clone(), CrossMode::Under),
            &mut RelativeStrengthIndexComponent::new(ctx.clone(), 14),
            Some(70.0),
            &expected,
        );
    }
}
