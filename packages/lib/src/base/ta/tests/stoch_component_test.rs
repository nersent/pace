#[cfg(test)]
mod tests {
    use crate::base::{
        components::{
            component_context::ComponentContext,
            testing::{ComponentTestSnapshot, Fixture},
        },
        ta::stoch_component::StochComponent,
    };

    fn format_path(path: &str) -> String {
        format!("base/ta/tests/fixtures/stoch/{}", path)
    }

    fn _test_close_high_low(
        cctx: &mut ComponentContext,
        target: &mut StochComponent,
        expected: &[Option<f64>],
    ) {
        let mut snapshot = ComponentTestSnapshot::<f64>::new();
        for cctx in cctx {
            let ctx = cctx.get();
            let ouptut = target.next(ctx.close(), ctx.high(), ctx.low());
            snapshot.push(ouptut);
        }
        snapshot.assert(expected);
    }

    fn _test_close_close_close(
        cctx: &mut ComponentContext,
        target: &mut StochComponent,
        expected: &[Option<f64>],
    ) {
        let mut snapshot = ComponentTestSnapshot::<f64>::new();
        for cctx in cctx {
            let ctx = cctx.get();
            let ouptut = target.next(ctx.close(), ctx.close(), ctx.close());
            snapshot.push(ouptut);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn length_14_close_high_low() {
        let (_df, ctx, expected) = Fixture::load(&format_path("length_14_close_high_low.csv"));
        _test_close_high_low(
            &mut ctx.clone(),
            &mut StochComponent::new(ctx.clone(), 14),
            &expected,
        );
    }

    #[test]
    fn length_1_close_high_low() {
        let (_df, ctx, expected) = Fixture::load(&format_path("length_1_close_high_low.csv"));
        _test_close_high_low(
            &mut ctx.clone(),
            &mut StochComponent::new(ctx.clone(), 1),
            &expected,
        );
    }

    #[test]
    fn length_2_close_high_low() {
        let (_df, ctx, expected) = Fixture::load(&format_path("length_2_close_high_low.csv"));
        _test_close_high_low(
            &mut ctx.clone(),
            &mut StochComponent::new(ctx.clone(), 2),
            &expected,
        );
    }

    #[test]
    fn length_3_close_high_low() {
        let (_df, ctx, expected) = Fixture::load(&format_path("length_3_close_high_low.csv"));
        _test_close_high_low(
            &mut ctx.clone(),
            &mut StochComponent::new(ctx.clone(), 3),
            &expected,
        );
    }

    #[test]
    fn length_1_close_close_close() {
        let (_df, ctx, expected) = Fixture::load(&format_path("length_1_close_close_close.csv"));
        _test_close_close_close(
            &mut ctx.clone(),
            &mut StochComponent::new(ctx.clone(), 1),
            &expected,
        );
    }

    #[test]
    fn length_2_close_close_close() {
        let (_df, ctx, expected) = Fixture::load(&format_path("length_2_close_close_close.csv"));
        _test_close_close_close(
            &mut ctx.clone(),
            &mut StochComponent::new(ctx.clone(), 2),
            &expected,
        );
    }

    #[test]
    fn length_3_close_close_close() {
        let (_df, ctx, expected) = Fixture::load(&format_path("length_3_close_close_close.csv"));
        _test_close_close_close(
            &mut ctx.clone(),
            &mut StochComponent::new(ctx.clone(), 3),
            &expected,
        );
    }

    #[test]
    fn length_14_close_close_close() {
        let (_df, ctx, expected) = Fixture::load(&format_path("length_14_close_close_close.csv"));
        _test_close_close_close(
            &mut ctx.clone(),
            &mut StochComponent::new(ctx.clone(), 14),
            &expected,
        );
    }
}
