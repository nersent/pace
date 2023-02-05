#[cfg(test)]
mod tests {
    use crate::base::{
        components::{
            component_context::ComponentContext,
            testing::{ComponentTestSnapshot, Fixture},
        },
        ta::change_component::ChangeComponent,
    };

    fn format_path(path: &str) -> String {
        format!("base/ta/tests/fixtures/change/{}", path)
    }

    fn _test(cctx: &mut ComponentContext, target: &mut ChangeComponent, expected: &[Option<f64>]) {
        let mut snapshot = ComponentTestSnapshot::<f64>::new();
        for cctx in cctx {
            let ctx = cctx.get();
            let ouptut = target.next(ctx.close());
            snapshot.push(ouptut);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn length_1_close() {
        let (_df, ctx, expected) = Fixture::load(&format_path("length_1_close.csv"));
        _test(
            &mut ctx.clone(),
            &mut ChangeComponent::new(ctx.clone(), 1),
            &expected,
        );
    }

    #[test]
    fn length_2_close() {
        let (_df, ctx, expected) = Fixture::load(&format_path("length_2_close.csv"));
        _test(
            &mut ctx.clone(),
            &mut ChangeComponent::new(ctx.clone(), 2),
            &expected,
        );
    }

    #[test]
    fn length_3_close() {
        let (_df, ctx, expected) = Fixture::load(&format_path("length_3_close.csv"));
        _test(
            &mut ctx.clone(),
            &mut ChangeComponent::new(ctx.clone(), 3),
            &expected,
        );
    }

    #[test]
    fn length_365_close() {
        let (_df, ctx, expected) = Fixture::load(&format_path("length_365_close.csv"));
        _test(
            &mut ctx.clone(),
            &mut ChangeComponent::new(ctx.clone(), 365),
            &expected,
        );
    }
}
