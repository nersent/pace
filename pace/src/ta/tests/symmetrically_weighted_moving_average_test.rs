#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{
        core::incremental::Incremental,
        polars::series::SeriesCastUtils,
        ta::symmetrically_weighted_moving_average::Swma,
        testing::{
            array_snapshot::ArraySnapshot,
            fixture::{DataFrameFixtureUtils, Fixture},
            pace::format_pace_fixture_path,
        },
    };

    fn format_path(path: &str) -> PathBuf {
        format_pace_fixture_path(&format!("tests/ta/swma/{}", path))
    }

    fn _test(target: &mut Swma, expected: &[f64]) {
        let mut snapshot = ArraySnapshot::<f64>::new();
        for _ in target.ctx.clone() {
            let output = target.next(target.ctx.bar.close());
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    // fn _test_with_rsi(
    //     cctx: &mut ComponentContext,
    //     target: &mut SymmetricallyWeightedMovingAverageComponent,
    //     target_rsi: &mut RelativeStrengthIndexIndicator,
    //     expected: &[f64],
    // ) {
    //     let mut snapshot = ArraySnapshot::<f64::new();
    //     for _ in target.ctx.clone() {
    //         let rsi = target_rsi.next();
    //         let output = target.next(rsi.rsi);
    //         snapshot.push(output);
    //     }
    //     snapshot.assert(expected);
    // }

    #[test]
    fn close() {
        let (df, ctx) = Fixture::load(&format_path("close.csv"));
        _test(&mut Swma::new(ctx.clone()), &df.test_target());
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
