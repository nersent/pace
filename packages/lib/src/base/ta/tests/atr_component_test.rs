#[cfg(test)]
mod tests {
    use crate::base::{
        components::{
            component_context::ComponentContext,
            testing::{ComponentTestSnapshot, Fixture},
        },
        ta::atr_component::AverageTrueRangeComponent,
    };

    fn format_path(path: &str) -> String {
        format!("base/ta/tests/fixtures/atr/{}", path)
    }

    fn _test(
        cctx: &mut ComponentContext,
        target: &mut AverageTrueRangeComponent,
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
    fn length_14() {
        let (_df, ctx, expected) = Fixture::load(&format_path("length_14.csv"));
        _test(
            &mut ctx.clone(),
            &mut AverageTrueRangeComponent::new(ctx.clone(), 14),
            &expected,
        );
    }

    #[test]
    fn length_1() {
        let (_df, ctx, expected) = Fixture::load(&format_path("length_1.csv"));
        _test(
            &mut ctx.clone(),
            &mut AverageTrueRangeComponent::new(ctx.clone(), 1),
            &expected,
        );
    }

    #[test]
    fn length_2() {
        let (_df, ctx, expected) = Fixture::load(&format_path("length_2.csv"));
        _test(
            &mut ctx.clone(),
            &mut AverageTrueRangeComponent::new(ctx.clone(), 2),
            &expected,
        );
    }
}
