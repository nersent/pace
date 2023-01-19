#[cfg(test)]
mod tests {
    use crate::components::{
        component_context::ComponentContext,
        implicit::recursive::recursive_rma::RecursiveRMA,
        utils::testing::{load_test_artifact_with_target, ComponentTestSnapshot},
    };

    fn _test(cctx: &mut ComponentContext, target: &mut RecursiveRMA, expected: &[Option<f64>]) {
        let mut snapshot = ComponentTestSnapshot::<f64>::new();
        for cctx in cctx {
            let output = target.next(cctx.get().close());
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn test_rma_btc_1d_length2_close() {
        let (_df, ctx, expected) =
            load_test_artifact_with_target("implicit/recursive/rma/btc_1d_length_2_close.csv");
        _test(
            &mut ctx.clone(),
            &mut RecursiveRMA::new(ctx.clone(), 2),
            &expected,
        );
    }

    #[test]
    fn test_rma_btc_1d_length3_close() {
        let (_df, ctx, expected) =
            load_test_artifact_with_target("implicit/recursive/rma/btc_1d_length_3_close.csv");
        _test(
            &mut ctx.clone(),
            &mut RecursiveRMA::new(ctx.clone(), 3),
            &expected,
        );
    }

    #[test]
    fn test_rma_btc_1d_length7_close() {
        let (_df, ctx, expected) =
            load_test_artifact_with_target("implicit/recursive/rma/btc_1d_length_7_close.csv");
        _test(
            &mut ctx.clone(),
            &mut RecursiveRMA::new(ctx.clone(), 7),
            &expected,
        );
    }

    #[test]
    fn test_rma_btc_1d_length_14_close() {
        let (_df, ctx, expected) =
            load_test_artifact_with_target("implicit/recursive/rma/btc_1d_length_14_close.csv");
        _test(
            &mut ctx.clone(),
            &mut RecursiveRMA::new(ctx.clone(), 14),
            &expected,
        );
    }

    #[test]
    fn test_rma_btc_1d_length_350_close() {
        let (_df, ctx, expected) =
            load_test_artifact_with_target("implicit/recursive/rma/btc_1d_length_350_close.csv");
        _test(
            &mut ctx.clone(),
            &mut RecursiveRMA::new(ctx.clone(), 350),
            &expected,
        );
    }
}
