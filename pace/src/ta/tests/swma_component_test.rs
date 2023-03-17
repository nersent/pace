#[cfg(test)]
mod tests {
    use crate::{
        components::component::Component,
        ta::{ema_component::EmaComponent, swma_component::SwmaComponent},
        testing::{
            array_snapshot::ArraySnapshot,
            fixture::{DataFrameFixtureUtils, Fixture},
        },
    };

    fn format_path(path: &str) -> String {
        format!("tests/ta/swma/{}", path)
    }

    fn _test(target: &mut SwmaComponent, expected: &[Option<f64>]) {
        let mut snapshot = ArraySnapshot::<Option<f64>>::new();
        for _ in target.ctx.clone() {
            let output = target.next(target.ctx.close());
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
    //     let mut snapshot = ArraySnapshot::<Option<f64>::new();
    //     for _ in target.ctx.clone() {
    //         let rsi = target_rsi.next();
    //         let output = target.next(rsi.rsi);
    //         snapshot.push(output);
    //     }
    //     snapshot.assert(expected);
    // }

    #[test]
    fn close() {
        let (df, ctx) = Fixture::load_ctx(&format_path("close.csv"));
        _test(&mut SwmaComponent::new(ctx.clone()), &df.test_target());
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
    //                     crate::components::source::SrcKind::Close,
    //                 ),
    //             },
    //         ),
    //         &expected,
    //     );
    // }
}
