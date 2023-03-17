#[cfg(test)]
mod tests {

    use crate::{
        components::{component::Component, component_context::ComponentContext},
        ta::{
            ema_component::EmaComponent, rsi_component::RsiComponent, sma_component::SmaComponent,
        },
        testing::{
            array_snapshot::ArraySnapshot,
            fixture::{DataFrameFixtureUtils, Fixture},
        },
    };

    fn format_path(path: &str) -> String {
        format!("tests/ta/rsi/{}", path)
    }

    fn _test(target: &mut RsiComponent, expected: &[Option<f64>]) {
        let mut snapshot = ArraySnapshot::<Option<f64>>::new();
        for _ in target.ctx.clone() {
            let output = target.next(target.ctx.close());
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn length_2_close() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_2_close.csv"));
        _test(&mut RsiComponent::new(ctx.clone(), 2), &df.test_target());
    }

    #[test]
    fn length_3_close() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_3_close.csv"));
        _test(&mut RsiComponent::new(ctx.clone(), 3), &df.test_target());
    }

    #[test]
    fn length_7_close() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_7_close.csv"));
        _test(&mut RsiComponent::new(ctx.clone(), 7), &df.test_target());
    }

    #[test]
    fn length_14_close() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_14_close.csv"));
        _test(&mut RsiComponent::new(ctx.clone(), 14), &df.test_target());
    }

    #[test]
    fn length_350_close() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_350_close.csv"));
        _test(&mut RsiComponent::new(ctx.clone(), 350), &df.test_target());
    }

    fn _test_with_sma(
        target: &mut RsiComponent,
        target_sma: &mut SmaComponent,
        expected: &[Option<f64>],
    ) {
        let mut snapshot = ArraySnapshot::<Option<f64>>::new();
        for _ in target.ctx.clone() {
            let output_sma = target_sma.next(target.ctx.close());
            let output_rsi = target.next(output_sma);
            snapshot.push(output_rsi);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn length_2_with_sma_length_14_close() {
        let (df, ctx) =
            Fixture::load_ctx(&format_path("sma/length_2_with_sma_length_14_close.csv"));
        _test_with_sma(
            &mut RsiComponent::new(ctx.clone(), 2),
            &mut SmaComponent::new(ctx.clone(), 14),
            &df.test_target(),
        );
    }

    #[test]
    fn length_14_with_sma_length_2_close() {
        let (df, ctx) =
            Fixture::load_ctx(&format_path("sma/length_14_with_sma_length_2_close.csv"));
        _test_with_sma(
            &mut RsiComponent::new(ctx.clone(), 14),
            &mut SmaComponent::new(ctx.clone(), 2),
            &df.test_target(),
        );
    }

    #[test]
    fn length_14_with_sma_length_14_close() {
        let (df, ctx) =
            Fixture::load_ctx(&format_path("sma/length_14_with_sma_length_14_close.csv"));
        _test_with_sma(
            &mut RsiComponent::new(ctx.clone(), 14),
            &mut SmaComponent::new(ctx.clone(), 14),
            &df.test_target(),
        );
    }
}
