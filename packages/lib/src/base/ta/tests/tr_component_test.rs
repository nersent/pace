#[cfg(test)]
mod tests {
    use crate::base::{
        components::{
            component_context::ComponentContext,
            testing::{ComponentTestSnapshot, Fixture},
        },
        ta::tr_component::TrueRangeComponent,
    };

    fn format_path(path: &str) -> String {
        format!("base/ta/tests/fixtures/tr/{}", path)
    }

    fn _test_true_range(
        cctx: &mut ComponentContext,
        target: &mut TrueRangeComponent,
        expected: &[Option<f64>],
    ) {
        let mut snapshot = ComponentTestSnapshot::<f64>::new();
        for cctx in cctx {
            let output = target.next();
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn without_handle_na() {
        let (_df, ctx, expected) = Fixture::load(&format_path("without_handle.csv"));
        _test_true_range(
            &mut ctx.clone(),
            &mut TrueRangeComponent::new(ctx.clone(), false),
            &expected,
        );
    }

    #[test]
    fn with_handle_na() {
        let (_df, ctx, expected) = Fixture::load(&format_path("with_handle.csv"));
        _test_true_range(
            &mut ctx.clone(),
            &mut TrueRangeComponent::new(ctx.clone(), true),
            &expected,
        );
    }
}
