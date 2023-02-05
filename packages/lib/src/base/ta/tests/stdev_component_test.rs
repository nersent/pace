#[cfg(test)]
mod tests {
    use crate::base::{
        components::{
            component_context::ComponentContext,
            testing::{ComponentTestSnapshot, Fixture},
        },
        ta::stdev_component::StandardDeviationComponent,
    };

    fn format_path(path: &str) -> String {
        format!("base/ta/tests/fixtures/stdev/{}", path)
    }

    fn _test(
        cctx: &mut ComponentContext,
        target: &mut StandardDeviationComponent,
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
    fn unbiased_length_1_close() {
        let (_df, ctx, expected) = Fixture::load(&format_path("length_1_unbiased.csv"));
        _test(
            &mut ctx.clone(),
            &mut StandardDeviationComponent::new(ctx.clone(), 1, false),
            &expected,
        );
    }

    #[test]
    fn unbiased_length_2_close() {
        let (_df, ctx, expected) = Fixture::load(&format_path("length_2_unbiased.csv"));
        _test(
            &mut ctx.clone(),
            &mut StandardDeviationComponent::new(ctx.clone(), 2, false),
            &expected,
        );
    }

    #[test]
    fn unbiased_length_3_close() {
        let (_df, ctx, expected) = Fixture::load(&format_path("length_3_unbiased.csv"));
        _test(
            &mut ctx.clone(),
            &mut StandardDeviationComponent::new(ctx.clone(), 3, false),
            &expected,
        );
    }

    #[test]
    fn unbiased_length_14_close() {
        let (_df, ctx, expected) = Fixture::load(&format_path("length_14_unbiased.csv"));
        _test(
            &mut ctx.clone(),
            &mut StandardDeviationComponent::new(ctx.clone(), 14, false),
            &expected,
        );
    }

    #[test]
    fn unbiased_length_365_close() {
        let (_df, ctx, expected) = Fixture::load(&format_path("length_365_unbiased.csv"));
        _test(
            &mut ctx.clone(),
            &mut StandardDeviationComponent::new(ctx.clone(), 365, false),
            &expected,
        );
    }

    #[test]
    fn biased_length_1_close() {
        let (_df, ctx, expected) = Fixture::load(&format_path("length_1_biased.csv"));
        _test(
            &mut ctx.clone(),
            &mut StandardDeviationComponent::new(ctx.clone(), 1, true),
            &expected,
        );
    }

    #[test]
    fn biased_length_2_close() {
        let (_df, ctx, expected) = Fixture::load(&format_path("length_2_biased.csv"));
        _test(
            &mut ctx.clone(),
            &mut StandardDeviationComponent::new(ctx.clone(), 2, true),
            &expected,
        );
    }

    #[test]
    fn biased_length_3_close() {
        let (_df, ctx, expected) = Fixture::load(&format_path("length_3_biased.csv"));
        _test(
            &mut ctx.clone(),
            &mut StandardDeviationComponent::new(ctx.clone(), 3, true),
            &expected,
        );
    }

    #[test]
    fn biased_length_14_close() {
        let (_df, ctx, expected) = Fixture::load(&format_path("length_14_biased.csv"));
        _test(
            &mut ctx.clone(),
            &mut StandardDeviationComponent::new(ctx.clone(), 14, true),
            &expected,
        );
    }

    #[test]
    fn biased_length_365_close() {
        let (_df, ctx, expected) = Fixture::load(&format_path("length_365_biased.csv"));
        _test(
            &mut ctx.clone(),
            &mut StandardDeviationComponent::new(ctx.clone(), 365, true),
            &expected,
        );
    }
}
