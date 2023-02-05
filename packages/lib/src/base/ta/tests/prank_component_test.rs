#[cfg(test)]
mod tests {
    use crate::base::{
        components::{
            component_context::ComponentContext,
            testing::{ComponentTestSnapshot, Fixture},
        },
        ta::{prank_component::PercentRankComponent, roc_component::RateOfChangeComponent},
    };

    fn format_path(path: &str) -> String {
        format!("base/ta/tests/fixtures/prank/{}", path)
    }

    fn _test(
        cctx: &mut ComponentContext,
        target: &mut PercentRankComponent,
        expected: &[Option<f64>],
    ) {
        let mut snapshot = ComponentTestSnapshot::<f64>::new();
        for cctx in cctx {
            let ctx = cctx.get();
            let output = target.next(ctx.close());
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn length_1_close() {
        let (_df, ctx, expected) = Fixture::load(&format_path("length_1_close.csv"));
        _test(
            &mut ctx.clone(),
            &mut PercentRankComponent::new(ctx.clone(), 1),
            &expected,
        );
    }

    #[test]
    fn length_2_close() {
        let (_df, ctx, expected) = Fixture::load(&format_path("length_2_close.csv"));
        _test(
            &mut ctx.clone(),
            &mut PercentRankComponent::new(ctx.clone(), 2),
            &expected,
        );
    }

    #[test]
    fn length_3_close() {
        let (_df, ctx, expected) = Fixture::load(&format_path("length_3_close.csv"));
        _test(
            &mut ctx.clone(),
            &mut PercentRankComponent::new(ctx.clone(), 3),
            &expected,
        );
    }

    #[test]
    fn length_14_close() {
        let (_df, ctx, expected) = Fixture::load(&format_path("length_14_close.csv"));
        _test(
            &mut ctx.clone(),
            &mut PercentRankComponent::new(ctx.clone(), 14),
            &expected,
        );
    }

    #[test]
    fn length_100_close() {
        let (_df, ctx, expected) = Fixture::load(&format_path("length_100_close.csv"));
        _test(
            &mut ctx.clone(),
            &mut PercentRankComponent::new(ctx.clone(), 100),
            &expected,
        );
    }

    #[test]
    fn length_365_close() {
        let (_df, ctx, expected) = Fixture::load(&format_path("length_365_close.csv"));
        _test(
            &mut ctx.clone(),
            &mut PercentRankComponent::new(ctx.clone(), 365),
            &expected,
        );
    }

    fn _test_with_roc(
        cctx: &mut ComponentContext,
        target: &mut PercentRankComponent,
        target_roc: &mut RateOfChangeComponent,
        expected: &[Option<f64>],
    ) {
        let mut snapshot = ComponentTestSnapshot::<f64>::new();
        for cctx in cctx {
            let ctx = cctx.get();
            let output_roc = target_roc.next(ctx.close());
            let output = target.next(output_roc);
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn length_14_with_roc_length_7_close() {
        let (_df, ctx, expected) =
            Fixture::load(&format_path("roc/length_14_with_roc_length_7_close.csv"));
        _test_with_roc(
            &mut ctx.clone(),
            &mut PercentRankComponent::new(ctx.clone(), 14),
            &mut RateOfChangeComponent::new(ctx.clone(), 7),
            &expected,
        );
    }

    #[test]
    fn length_100_with_roc_length_7_close() {
        let (_df, ctx, expected) =
            Fixture::load(&format_path("roc/length_100_with_roc_length_7_close.csv"));
        _test_with_roc(
            &mut ctx.clone(),
            &mut PercentRankComponent::new(ctx.clone(), 100),
            &mut RateOfChangeComponent::new(ctx.clone(), 7),
            &expected,
        );
    }

    #[test]
    fn length_100_with_roc_length_2_close() {
        let (_df, ctx, expected) =
            Fixture::load(&format_path("roc/length_100_with_roc_length_2_close.csv"));
        _test_with_roc(
            &mut ctx.clone(),
            &mut PercentRankComponent::new(ctx.clone(), 100),
            &mut RateOfChangeComponent::new(ctx.clone(), 2),
            &expected,
        );
    }

    #[test]
    fn length_100_with_roc_length_1_close() {
        let (_df, ctx, expected) =
            Fixture::load(&format_path("roc/length_100_with_roc_length_1_close.csv"));
        _test_with_roc(
            &mut ctx.clone(),
            &mut PercentRankComponent::new(ctx.clone(), 100),
            &mut RateOfChangeComponent::new(ctx.clone(), 1),
            &expected,
        );
    }

    // #[test]
    // fn test_recursive_rank_7_length_with_rate_of_change_14_length_btc_1d_close() {
    //     let (_df, ctx, expected) =
    //         Fixture::load("components/change/tests/fixtures/prank/roc/btc_1d_prank_length_7_roc_length_14_close.csv");
    //     _test_with_roc(
    //         &mut ctx.clone(),
    //         &mut RecursivePercentRank::new(ctx.clone(), 7),
    //         &mut RecursiveRateOfChange::new(ctx.clone(), 14),
    //         &expected,
    //     );
    // }

    // #[test]
    // fn test_recursive_rank_7_length_with_rate_of_change_1_length_btc_1d_close() {
    //     let (_df, ctx, expected) =
    //         Fixture::load("components/change/tests/fixtures/prank/roc/btc_1d_prank_length_7_roc_length_1_close.csv");
    //     _test_with_roc(
    //         &mut ctx.clone(),
    //         &mut RecursivePercentRank::new(ctx.clone(), 7),
    //         &mut RecursiveRateOfChange::new(ctx.clone(), 1),
    //         &expected,
    //     );
    // }
}
