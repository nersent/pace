#[cfg(test)]
mod tests {
    use crate::{
        components::{component_context::ComponentContext, testing::ComponentTestSnapshot},
        ta::{
            moving_average::sma_component::SimpleMovingAverageComponent,
            relative_strength_index::rsi_component::RelativeStrengthIndexComponent,
        },
        testing::fixture::Fixture,
    };

    fn _test(
        cctx: &mut ComponentContext,
        target: &mut RelativeStrengthIndexComponent,
        expected: &[Option<f64>],
    ) {
        let mut snapshot = ComponentTestSnapshot::<f64>::new();
        for cctx in cctx {
            let output = target.next(cctx.get().close());
            snapshot.push(output.rsi);
        }
        snapshot.assert(expected);
    }

    fn _test_with_sma(
        cctx: &mut ComponentContext,
        target_rsi: &mut RelativeStrengthIndexComponent,
        target_sma: &mut SimpleMovingAverageComponent,
        expected: &[Option<f64>],
    ) {
        let mut snapshot = ComponentTestSnapshot::<f64>::new();
        for cctx in cctx {
            let output_sma = target_sma.next(cctx.get().close());
            let output_rsi = target_rsi.next(output_sma);
            snapshot.push(output_rsi.rsi);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn test_rsi_btc_1d_length2_close() {
        let (_df, ctx, expected) = Fixture::load(
            "ta/relative_strength_index/tests/fixtures/rsi_component/btc_1d_length_2_close.csv",
        );
        _test(
            &mut ctx.clone(),
            &mut RelativeStrengthIndexComponent::new(ctx.clone(), 2),
            &expected,
        );
    }

    #[test]
    fn test_rsi_btc_1d_length3_close() {
        let (_df, ctx, expected) = Fixture::load(
            "ta/relative_strength_index/tests/fixtures/rsi_component/btc_1d_length_3_close.csv",
        );
        _test(
            &mut ctx.clone(),
            &mut RelativeStrengthIndexComponent::new(ctx.clone(), 3),
            &expected,
        );
    }

    #[test]
    fn test_rsi_btc_1d_length7_close() {
        let (_df, ctx, expected) = Fixture::load(
            "ta/relative_strength_index/tests/fixtures/rsi_component/btc_1d_length_7_close.csv",
        );
        _test(
            &mut ctx.clone(),
            &mut RelativeStrengthIndexComponent::new(ctx.clone(), 7),
            &expected,
        );
    }

    #[test]
    fn test_rsi_btc_1d_length_14_close() {
        let (_df, ctx, expected) = Fixture::load(
            "ta/relative_strength_index/tests/fixtures/rsi_component/btc_1d_length_14_close.csv",
        );
        _test(
            &mut ctx.clone(),
            &mut RelativeStrengthIndexComponent::new(ctx.clone(), 14),
            &expected,
        );
    }

    #[test]
    fn test_rsi_btc_1d_length_350_close() {
        let (_df, ctx, expected) = Fixture::load(
            "ta/relative_strength_index/tests/fixtures/rsi_component/btc_1d_length_350_close.csv",
        );
        _test(
            &mut ctx.clone(),
            &mut RelativeStrengthIndexComponent::new(ctx.clone(), 350),
            &expected,
        );
    }

    #[test]
    fn test_rsi_btc_1d_rsi_length_2_sma_length_14_close() {
        let (_df, ctx, expected) =
            Fixture::load("ta/relative_strength_index/tests/fixtures/rsi_component/sma/btc_1d_rsi_length_2_sma_length_14_close.csv");
        _test_with_sma(
            &mut ctx.clone(),
            &mut RelativeStrengthIndexComponent::new(ctx.clone(), 2),
            &mut SimpleMovingAverageComponent::new(ctx.clone(), 14),
            &expected,
        );
    }

    #[test]
    fn test_rsi_btc_1d_rsi_length_14_sma_length_2_close() {
        let (_df, ctx, expected) =
            Fixture::load("ta/relative_strength_index/tests/fixtures/rsi_component/sma/btc_1d_rsi_length_14_sma_length_2_close.csv");
        _test_with_sma(
            &mut ctx.clone(),
            &mut RelativeStrengthIndexComponent::new(ctx.clone(), 14),
            &mut SimpleMovingAverageComponent::new(ctx.clone(), 2),
            &expected,
        );
    }

    #[test]
    fn test_rsi_btc_1d_rsi_length_14_sma_length_14_close() {
        let (_df, ctx, expected) = Fixture::load(
            "ta/relative_strength_index/tests/fixtures/rsi_component/sma/btc_1d_rsi_length_14_sma_length_14_close.csv",
        );
        _test_with_sma(
            &mut ctx.clone(),
            &mut RelativeStrengthIndexComponent::new(ctx.clone(), 14),
            &mut SimpleMovingAverageComponent::new(ctx.clone(), 14),
            &expected,
        );
    }
}
