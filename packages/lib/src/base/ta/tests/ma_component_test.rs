#[cfg(test)]
mod tests {
    use crate::base::{
        asset::source::{Source, SourceKind},
        components::{
            component_context::ComponentContext,
            testing::{ComponentTestSnapshot, Fixture},
        },
        ta::{ma::MovingAverageKind, ma_component::MovingAverageComponent},
    };

    fn _test(
        cctx: &mut ComponentContext,
        target: &mut MovingAverageComponent,
        source: Source,
        expected: &[Option<f64>],
    ) {
        let mut snapshot = ComponentTestSnapshot::<f64>::new();
        for cctx in cctx {
            let output = target.next(source.get());
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn length_34_hl2() {
        let (_df, ctx, expected) = Fixture::load("base/ta/tests/fixtures/sma/length_34_hl2.csv");
        _test(
            &mut ctx.clone(),
            &mut MovingAverageComponent::new(ctx.clone(), 34, MovingAverageKind::SMA),
            Source::from_kind(ctx.clone(), SourceKind::HL2),
            &expected,
        );
    }
}
