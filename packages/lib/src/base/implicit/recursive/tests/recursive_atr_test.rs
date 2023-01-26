#[cfg(test)]
mod tests {
    use crate::base::{
        component_context::ComponentContext,
        implicit::recursive::{recursive_atr::RecursiveATR, recursive_sma::RecursiveSMA},
        utils::testing::{load_test_artifact_with_target, ComponentTestSnapshot},
    };

    fn _test(cctx: &mut ComponentContext, target: &mut RecursiveATR, expected: &[Option<f64>]) {
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
            load_test_artifact_with_target("implicit/recursive/atr/btc_1d_length_14.csv");
        _test(
            &mut ctx.clone(),
            &mut RecursiveATR::new(ctx.clone(), 14),
            &expected,
        );
    }
}
