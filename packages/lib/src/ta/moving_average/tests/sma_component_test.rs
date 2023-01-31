#[cfg(test)]
mod tests {
    use crate::{
        components::{
            component_context::ComponentContext, dev::stdev_component::StandardDeviationComponent,
            testing::ComponentTestSnapshot,
        },
        ta::moving_average::sma_component::SimpleMovingAverageComponent,
        testing::fixture::Fixture,
    };

    fn _test(
        cctx: &mut ComponentContext,
        target: &mut SimpleMovingAverageComponent,
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
    fn test_sma_btc_1d_length1_close() {
        let (_df, ctx, expected) =
            Fixture::load("ta/moving_average/tests/fixtures/sma/btc_1d_length_1_close.csv");
        _test(
            &mut ctx.clone(),
            &mut SimpleMovingAverageComponent::new(ctx.clone(), 1),
            &expected,
        );
    }

    #[test]
    fn test_sma_btc_1d_length2_close() {
        let (_df, ctx, expected) =
            Fixture::load("ta/moving_average/tests/fixtures/sma/btc_1d_length_2_close.csv");
        _test(
            &mut ctx.clone(),
            &mut SimpleMovingAverageComponent::new(ctx.clone(), 2),
            &expected,
        );
    }

    #[test]
    fn test_sma_btc_1d_length3_close() {
        let (_df, ctx, expected) =
            Fixture::load("ta/moving_average/tests/fixtures/sma/btc_1d_length_3_close.csv");
        _test(
            &mut ctx.clone(),
            &mut SimpleMovingAverageComponent::new(ctx.clone(), 3),
            &expected,
        );
    }

    #[test]
    fn test_sma_btc_1d_length7_close() {
        let (_df, ctx, expected) =
            Fixture::load("ta/moving_average/tests/fixtures/sma/btc_1d_length_7_close.csv");
        _test(
            &mut ctx.clone(),
            &mut SimpleMovingAverageComponent::new(ctx.clone(), 7),
            &expected,
        );
    }

    #[test]
    fn test_sma_btc_1d_length_14_close() {
        let (_df, ctx, expected) =
            Fixture::load("ta/moving_average/tests/fixtures/sma/btc_1d_length_14_close.csv");
        _test(
            &mut ctx.clone(),
            &mut SimpleMovingAverageComponent::new(ctx.clone(), 14),
            &expected,
        );
    }

    #[test]
    fn test_sma_btc_1d_length_350_close() {
        let (_df, ctx, expected) =
            Fixture::load("ta/moving_average/tests/fixtures/sma/btc_1d_length_350_close.csv");
        _test(
            &mut ctx.clone(),
            &mut SimpleMovingAverageComponent::new(ctx.clone(), 350),
            &expected,
        );
    }

    // #[test]
    // fn test_sma_btc_1d_sma_14_length_change_stdev_14_length_close() {
    //     let (_df, ctx, expected) =
    //         Fixture::load("ta/moving_average/tests/fixtures/sma/change/stdev/btc_1d_sma_14_length_change_stdev_14_length_close.csv");
    //     let mut target_sma = SimpleMovingAverageComponent::new(ctx.clone(), 14);
    //     let mut target_stdev = StandardDeviationComponent::new(ctx.clone(), 14, true);
    //     let mut prev_src: Option<f64> = None;
    //     let mut snapshot = ComponentTestSnapshot::<f64>::new();
    //     for cctx in ctx {
    //         let ctx = cctx.get();
    //         let src = ctx.close();
    //         let stdev = target_stdev.next(src);
    //         let src_change = match (src, prev_src) {
    //             (Some(src), Some(prev_src)) => Some(src - prev_src),
    //             _ => None,
    //         };
    //         let change: Option<f64> = match src_change {
    //             Some(src_change) => {
    //                 if src_change <= 0.0 {
    //                     Some(0.0)
    //                 } else {
    //                     stdev
    //                 }
    //             }
    //             _ => None,
    //         };
    //         let sma = target_sma.next(change);
    //         prev_src = src;
    //         snapshot.push(sma);
    //     }
    //     snapshot.assert(&expected);
    // }
}
