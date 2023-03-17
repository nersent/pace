#[cfg(test)]
mod tests {
    use crate::{
        components::{component::Component, component_context::ComponentContext},
        ta::{
            change_component::ChangeComponent, cross_over_component::CrossOverComponent,
            ema_component::EmaComponent, rsi_component::RsiComponent,
        },
        testing::{
            array_snapshot::ArraySnapshot,
            fixture::{DataFrameFixtureUtils, Fixture},
        },
    };

    fn format_path(path: &str) -> String {
        format!("tests/ta/cross/{}", path)
    }

    fn _test(
        target: &mut CrossOverComponent,
        target_rsi: &mut RsiComponent,
        threshold: Option<f64>,
        expected: &[Option<f64>],
    ) {
        let mut snapshot = ArraySnapshot::<Option<f64>>::new();
        for _ in target.ctx.clone() {
            let output_rsi = target_rsi.next(target.ctx.close());
            let output = target.next(output_rsi, threshold);
            snapshot.push(Some(if output { 1.0 } else { 0.0 }));
        }
        snapshot.assert(expected);
    }

    #[test]
    fn over_with_rsi_length_14_close() {
        let (df, ctx) = Fixture::load_ctx(&format_path("over/rsi/length_14_close.csv"));
        _test(
            &mut CrossOverComponent::new(ctx.clone()),
            &mut RsiComponent::new(ctx.clone(), 14),
            Some(30.0),
            &df.test_target(),
        );
    }
}