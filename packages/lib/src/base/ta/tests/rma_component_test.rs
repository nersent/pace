#[cfg(test)]
mod tests {
    use crate::base::{
        components::{
            component_context::ComponentContext,
            testing::{ComponentTestSnapshot, Fixture},
        },
        ta::rma_component::RunningMovingAverageComponent,
    };

    fn _test(
        cctx: &mut ComponentContext,
        target: &mut RunningMovingAverageComponent,
        expected: &[Option<f64>],
    ) {
        let mut snapshot = ComponentTestSnapshot::<f64>::new();
        for cctx in cctx {
            let output = target.next(cctx.get().close());
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    fn format_path(path: &str) -> String {
        format!("base/ta/tests/fixtures/rma/{}", path)
    }

    #[test]
    fn length_1_close() {
        let (_df, ctx, expected) = Fixture::load(&format_path("length_1_close.csv"));
        _test(
            &mut ctx.clone(),
            &mut RunningMovingAverageComponent::new(ctx.clone(), 1),
            &expected,
        );
    }

    #[test]
    fn length_2_close() {
        let (_df, ctx, expected) = Fixture::load(&format_path("length_2_close.csv"));
        _test(
            &mut ctx.clone(),
            &mut RunningMovingAverageComponent::new(ctx.clone(), 2),
            &expected,
        );
    }

    #[test]
    fn length_3_close() {
        let (_df, ctx, expected) = Fixture::load(&format_path("length_3_close.csv"));
        _test(
            &mut ctx.clone(),
            &mut RunningMovingAverageComponent::new(ctx.clone(), 3),
            &expected,
        );
    }

    #[test]
    fn length_7_close() {
        let (_df, ctx, expected) = Fixture::load(&format_path("length_7_close.csv"));
        _test(
            &mut ctx.clone(),
            &mut RunningMovingAverageComponent::new(ctx.clone(), 7),
            &expected,
        );
    }

    #[test]
    fn length_14_close() {
        let (_df, ctx, expected) = Fixture::load(&format_path("length_14_close.csv"));
        _test(
            &mut ctx.clone(),
            &mut RunningMovingAverageComponent::new(ctx.clone(), 14),
            &expected,
        );
    }

    #[test]
    fn length_350_close() {
        let (_df, ctx, expected) = Fixture::load(&format_path("length_350_close.csv"));
        _test(
            &mut ctx.clone(),
            &mut RunningMovingAverageComponent::new(ctx.clone(), 350),
            &expected,
        );
    }
}
