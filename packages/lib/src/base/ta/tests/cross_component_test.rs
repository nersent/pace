#[cfg(test)]
mod tests {
    use crate::base::{
        components::{
            component_context::ComponentContext,
            testing::{ComponentTestSnapshot, Fixture},
        },
        ta::{
            cross::CrossMode, cross_component::CrossComponent,
            rsi_component::RelativeStrengthIndexComponent,
        },
    };

    fn format_path(path: &str) -> String {
        format!("base/ta/tests/fixtures/cross/{}", path)
    }

    fn _test(
        cctx: &mut ComponentContext,
        target_cross: &mut CrossComponent,
        target_rsi: &mut RelativeStrengthIndexComponent,
        threshold: Option<f64>,
        expected: &[Option<f64>],
    ) {
        let mut snapshot = ComponentTestSnapshot::<f64>::new();
        for cctx in cctx {
            let output_rsi = target_rsi.next(cctx.get().close());
            let output = target_cross.next(output_rsi, threshold);
            snapshot.push(Some(if output { 1.0 } else { 0.0 }));
        }
        snapshot.assert(expected);
    }

    #[test]
    fn tover_with_rsi_length_14_close() {
        let (_df, ctx, expected) = Fixture::load(&format_path("over/rsi/length_14_close.csv"));
        _test(
            &mut ctx.clone(),
            &mut CrossComponent::new(ctx.clone(), CrossMode::Over),
            &mut RelativeStrengthIndexComponent::new(ctx.clone(), 14),
            Some(30.0),
            &expected,
        );
    }

    #[test]
    fn under_with_rsi_length_14_close() {
        let (_df, ctx, expected) = Fixture::load(&format_path("under/rsi/length_14_close.csv"));
        _test(
            &mut ctx.clone(),
            &mut CrossComponent::new(ctx.clone(), CrossMode::Under),
            &mut RelativeStrengthIndexComponent::new(ctx.clone(), 14),
            Some(70.0),
            &expected,
        );
    }
}
