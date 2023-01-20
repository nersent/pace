#[cfg(test)]
mod tests {
    use crate::base::{
        component_context::ComponentContext,
        implicit::recursive::{
            recursive_cross_under::RecursiveCrossUnder, recursive_rsi::RecursiveRSI,
            recursive_sma::RecursiveSMA,
        },
        utils::testing::{load_test_artifact_with_target, ComponentTestSnapshot},
    };

    fn _test(
        cctx: &mut ComponentContext,
        target_cross_under: &mut RecursiveCrossUnder,
        target_rsi: &mut RecursiveRSI,
        expected: &[Option<f64>],
    ) {
        let mut snapshot = ComponentTestSnapshot::<f64>::new();
        for cctx in cctx {
            let output_rsi = target_rsi.next(cctx.get().close());
            let output = target_cross_under.next(output_rsi.rsi, Some(70.0));
            snapshot.push(Some(if output { 1.0 } else { 0.0 }));
        }
        snapshot.assert(expected);
    }

    #[test]
    fn test_crossover_with_rsi_btc_1d_length14_close() {
        let (_df, ctx, expected) = load_test_artifact_with_target(
            "implicit/recursive/cross_under/rsi/btc_1d_length_14_close.csv",
        );
        _test(
            &mut ctx.clone(),
            &mut RecursiveCrossUnder::new(ctx.clone()),
            &mut RecursiveRSI::new(ctx.clone(), 14),
            &expected,
        );
    }
}
