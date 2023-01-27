#[cfg(test)]
mod tests {
    use crate::{
        components::{component_context::ComponentContext, testing::ComponentTestSnapshot},
        ta::true_range::atr_component::AverageTrueRangeComponent,
        testing::fixture::Fixture,
    };

    fn _test(
        cctx: &mut ComponentContext,
        target: &mut AverageTrueRangeComponent,
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
    fn test_atr_btc_1d_length_14() {
        let (_df, ctx, expected) =
            Fixture::load("ta/true_range/tests/fixtures/atr/btc_1d_length_14.csv");
        _test(
            &mut ctx.clone(),
            &mut AverageTrueRangeComponent::new(ctx.clone(), 14),
            &expected,
        );
    }
}
