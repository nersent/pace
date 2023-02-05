#[cfg(test)]
mod tests {
    use crate::base::{
        components::{
            component_context::ComponentContext,
            testing::{ComponentTestSnapshot, Fixture},
        },
        ta::{
            rsi_component::RelativeStrengthIndexComponent,
            sma_component::SimpleMovingAverageComponent,
        },
    };

    fn format_path(path: &str) -> String {
        format!("base/ta/tests/fixtures/rsi/{}", path)
    }

    fn _test(
        cctx: &mut ComponentContext,
        target: &mut RelativeStrengthIndexComponent,
        expected: &[Option<f64>],
    ) {
        let mut snapshot = ComponentTestSnapshot::<f64>::new();
        for cctx in cctx {
            let output = target.next(cctx.get().close());
            snapshot.push(output);
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
            snapshot.push(output_rsi);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn length_2_close() {
        let (_df, ctx, expected) = Fixture::load(&format_path("length_2_close.csv"));
        _test(
            &mut ctx.clone(),
            &mut RelativeStrengthIndexComponent::new(ctx.clone(), 2),
            &expected,
        );
    }

    #[test]
    fn length_3_close() {
        let (_df, ctx, expected) = Fixture::load(&format_path("length_3_close.csv"));
        _test(
            &mut ctx.clone(),
            &mut RelativeStrengthIndexComponent::new(ctx.clone(), 3),
            &expected,
        );
    }

    #[test]
    fn length_7_close() {
        let (_df, ctx, expected) = Fixture::load(&format_path("length_7_close.csv"));
        _test(
            &mut ctx.clone(),
            &mut RelativeStrengthIndexComponent::new(ctx.clone(), 7),
            &expected,
        );
    }

    #[test]
    fn length_14_close() {
        let (_df, ctx, expected) = Fixture::load(&format_path("length_14_close.csv"));
        _test(
            &mut ctx.clone(),
            &mut RelativeStrengthIndexComponent::new(ctx.clone(), 14),
            &expected,
        );
    }

    #[test]
    fn length_350_close() {
        let (_df, ctx, expected) = Fixture::load(&format_path("length_350_close.csv"));
        _test(
            &mut ctx.clone(),
            &mut RelativeStrengthIndexComponent::new(ctx.clone(), 350),
            &expected,
        );
    }

    #[test]
    fn length_2_with_sma_length_14_close() {
        let (_df, ctx, expected) =
            Fixture::load(&format_path("sma/length_2_with_sma_length_14_close.csv"));
        _test_with_sma(
            &mut ctx.clone(),
            &mut RelativeStrengthIndexComponent::new(ctx.clone(), 2),
            &mut SimpleMovingAverageComponent::new(ctx.clone(), 14),
            &expected,
        );
    }

    #[test]
    fn length_14_with_sma_length_2_close() {
        let (_df, ctx, expected) =
            Fixture::load(&format_path("sma/length_14_with_sma_length_2_close.csv"));
        _test_with_sma(
            &mut ctx.clone(),
            &mut RelativeStrengthIndexComponent::new(ctx.clone(), 14),
            &mut SimpleMovingAverageComponent::new(ctx.clone(), 2),
            &expected,
        );
    }

    #[test]
    fn length_14_with_sma_length_14_close() {
        let (_df, ctx, expected) =
            Fixture::load(&format_path("sma/length_14_with_sma_length_14_close.csv"));
        _test_with_sma(
            &mut ctx.clone(),
            &mut RelativeStrengthIndexComponent::new(ctx.clone(), 14),
            &mut SimpleMovingAverageComponent::new(ctx.clone(), 14),
            &expected,
        );
    }
}
