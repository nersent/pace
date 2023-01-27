#[cfg(test)]
mod tests {
    use crate::{
        components::{component_context::ComponentContext, testing::ComponentTestSnapshot},
        ta::moving_average::ema_component::ExponentialMovingAverageComponent,
        testing::fixture::Fixture,
    };

    fn _test(
        cctx: &mut ComponentContext,
        target: &mut ExponentialMovingAverageComponent,
        expected: &[Option<f64>],
    ) {
        let mut snapshot = ComponentTestSnapshot::<f64>::new();
        for cctx in cctx {
            let output = target.next(cctx.get().close());
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn test_ema_btc_1d_length2_close() {
        let (_df, ctx, expected) =
            Fixture::load("ta/moving_average/tests/fixtures/ema/btc_1d_length_2_close.csv");
        _test(
            &mut ctx.clone(),
            &mut ExponentialMovingAverageComponent::new(ctx.clone(), 2),
            &expected,
        );
    }

    #[test]
    fn test_ema_btc_1d_length3_close() {
        let (_df, ctx, expected) =
            Fixture::load("ta/moving_average/tests/fixtures/ema/btc_1d_length_3_close.csv");
        _test(
            &mut ctx.clone(),
            &mut ExponentialMovingAverageComponent::new(ctx.clone(), 3),
            &expected,
        );
    }

    #[test]
    fn test_ema_btc_1d_length7_close() {
        let (_df, ctx, expected) =
            Fixture::load("ta/moving_average/tests/fixtures/ema/btc_1d_length_7_close.csv");
        _test(
            &mut ctx.clone(),
            &mut ExponentialMovingAverageComponent::new(ctx.clone(), 7),
            &expected,
        );
    }

    #[test]
    fn test_ema_btc_1d_length_14_close() {
        let (_df, ctx, expected) =
            Fixture::load("ta/moving_average/tests/fixtures/ema/btc_1d_length_14_close.csv");
        _test(
            &mut ctx.clone(),
            &mut ExponentialMovingAverageComponent::new(ctx.clone(), 14),
            &expected,
        );
    }

    #[test]
    fn test_ema_btc_1d_length_350_close() {
        let (_df, ctx, expected) =
            Fixture::load("ta/moving_average/tests/fixtures/ema/btc_1d_length_350_close.csv");
        _test(
            &mut ctx.clone(),
            &mut ExponentialMovingAverageComponent::new(ctx.clone(), 350),
            &expected,
        );
    }
}
