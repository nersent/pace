#[cfg(test)]
mod tests {
    use crate::{
        components::component::Component,
        ta::{
            change_component::ChangeComponent, cross::CrossMode,
            cross_threshold_component::CrossThresholdComponent, ema_component::EmaComponent,
            rsi_component::RsiComponent,
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
        target: &mut CrossThresholdComponent,
        target_rsi: &mut RsiComponent,
        mode: CrossMode,
        expected: &[Option<f64>],
    ) {
        let mut snapshot = ArraySnapshot::<Option<f64>>::new();
        for _ in target.ctx.clone() {
            let output_rsi = target_rsi.next(target.ctx.close());
            let output = target.next(output_rsi);
            let output = match output {
                Some(output) => output == mode,
                None => false,
            };
            let output = if output { 1.0 } else { 0.0 };
            snapshot.push(Some(output));
        }
        snapshot.assert(expected);
    }

    #[test]
    fn over_with_rsi_length_14_close() {
        let (df, ctx) = Fixture::load_ctx(&format_path("over/rsi/length_14_close.csv"));
        _test(
            &mut CrossThresholdComponent::new(ctx.clone(), 30.0),
            &mut RsiComponent::new(ctx.clone(), 14),
            CrossMode::Over,
            &df.test_target(),
        );
    }

    #[test]
    fn under_with_rsi_length_14_close() {
        let (df, ctx) = Fixture::load_ctx(&format_path("under/rsi/length_14_close.csv"));
        _test(
            &mut CrossThresholdComponent::new(ctx.clone(), 70.0),
            &mut RsiComponent::new(ctx.clone(), 14),
            CrossMode::Under,
            &df.test_target(),
        );
    }
}
