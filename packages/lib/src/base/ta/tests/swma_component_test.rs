#[cfg(test)]
mod tests {
    use crate::base::{
        components::{
            component_context::ComponentContext,
            testing::{ComponentTestSnapshot, Fixture},
        },
        ta::swma_component::SymmetricallyWeightedMovingAverageComponent,
    };

    fn format_path(path: &str) -> String {
        format!("base/ta/tests/fixtures/swma/{}", path)
    }

    fn _test(
        cctx: &mut ComponentContext,
        target: &mut SymmetricallyWeightedMovingAverageComponent,
        expected: &[Option<f64>],
    ) {
        let mut snapshot = ComponentTestSnapshot::<f64>::new();
        for cctx in cctx {
            let output = target.next(cctx.get().close());
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    // fn _test_with_rsi(
    //     cctx: &mut ComponentContext,
    //     target: &mut SymmetricallyWeightedMovingAverageComponent,
    //     target_rsi: &mut RelativeStrengthIndexIndicator,
    //     expected: &[Option<f64>],
    // ) {
    //     let mut snapshot = ComponentTestSnapshot::<f64>::new();
    //     for cctx in cctx {
    //         let rsi = target_rsi.next();
    //         let output = target.next(rsi.rsi);
    //         snapshot.push(output);
    //     }
    //     snapshot.assert(expected);
    // }

    #[test]
    fn close() {
        let (_df, ctx, expected) = Fixture::load(&format_path("close.csv"));
        _test(
            &mut ctx.clone(),
            &mut SymmetricallyWeightedMovingAverageComponent::new(ctx.clone()),
            &expected,
        );
    }

    // #[test]
    // fn test_swma_with_rsi_14_length_btc_1d_close() {
    //     let (_df, ctx, expected) = Fixture::load(
    //         "ta/moving_average/tests/fixtures/swma/rsi/btc_1d_rsi_length_60_close.csv",
    //     );
    //     _test_with_rsi(
    //         &mut ctx.clone(),
    //         &mut SymmetricallyWeightedMovingAverageComponent::new(ctx.clone()),
    //         &mut RelativeStrengthIndexIndicator::new(
    //             ctx.clone(),
    //             RelativeStrengthIndexIndicatorConfig {
    //                 length: 60,
    //                 src: Source::from_kind(
    //                     ctx.clone(),
    //                     crate::components::source::SourceKind::Close,
    //                 ),
    //             },
    //         ),
    //         &expected,
    //     );
    // }
}
