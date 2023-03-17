#[cfg(test)]
mod tests {
    use crate::{
        components::component::Component,
        ta::{
            change_component::ChangeComponent,
            cross_under_threshold_component::CrossUnderThresholdComponent,
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
        target: &mut CrossUnderThresholdComponent,
        target_rsi: &mut RsiComponent,
        expected: &[Option<f64>],
    ) {
        let mut snapshot = ArraySnapshot::<Option<f64>>::new();
        for _ in target.ctx.clone() {
            let output_rsi = target_rsi.next(target.ctx.close());
            let output = target.next(output_rsi);
            snapshot.push(Some(if output { 1.0 } else { 0.0 }));
        }
        snapshot.assert(expected);
    }

    #[test]
    fn under_with_rsi_length_14_close() {
        let (df, ctx) = Fixture::load_ctx(&format_path("under/rsi/length_14_close.csv"));
        _test(
            &mut CrossUnderThresholdComponent::new(ctx.clone(), 70.0),
            &mut RsiComponent::new(ctx.clone(), 14),
            &df.test_target(),
        );
    }
}