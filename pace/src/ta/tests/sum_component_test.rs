#[cfg(test)]
mod tests {
    use crate::{
        components::component::Component,
        ta::{
            atr_component::AtrComponent, ema_component::EmaComponent, sma_component::SmaComponent,
            sum_component::SumComponent,
        },
        testing::{
            array_snapshot::ArraySnapshot,
            fixture::{DataFrameFixtureUtils, Fixture},
        },
    };

    fn format_path(path: &str) -> String {
        format!("tests/ta/sum/{}", path)
    }

    fn _test(target: &mut SumComponent, expected: &[Option<f64>]) {
        let mut snapshot = ArraySnapshot::<Option<f64>>::new();
        for _ in target.ctx.clone() {
            let output = target.next(target.ctx.close());
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn length_1_close() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_1_close.csv"));
        _test(&mut SumComponent::new(ctx.clone(), 1), &df.test_target());
    }

    #[test]
    fn length_2_close() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_2_close.csv"));
        _test(&mut SumComponent::new(ctx.clone(), 2), &df.test_target());
    }

    #[test]
    fn length_3_close() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_3_close.csv"));
        _test(&mut SumComponent::new(ctx.clone(), 3), &df.test_target());
    }

    #[test]
    fn length_14_close() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_14_close.csv"));
        _test(&mut SumComponent::new(ctx.clone(), 14), &df.test_target());
    }

    #[test]
    fn length_365_close() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_365_close.csv"));
        _test(&mut SumComponent::new(ctx.clone(), 365), &df.test_target());
    }

    fn _test_with_atr(
        target: &mut SumComponent,
        target_atr: &mut AtrComponent,
        expected: &[Option<f64>],
    ) {
        let mut snapshot = ArraySnapshot::<Option<f64>>::new();
        for _ in target.ctx.clone() {
            let atr = target_atr.next(());
            let output = target.next(atr);
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn length_1_with_atr_length_14() {
        let (df, ctx) = Fixture::load_ctx(&format_path("atr/length_1_with_atr_length_14.csv"));
        _test_with_atr(
            &mut SumComponent::new(ctx.clone(), 1),
            &mut AtrComponent::new(ctx.clone(), 14),
            &df.test_target(),
        );
    }

    #[test]
    fn length_14_with_atr_length_14() {
        let (df, ctx) = Fixture::load_ctx(&format_path("atr/length_14_with_atr_length_14.csv"));
        _test_with_atr(
            &mut SumComponent::new(ctx.clone(), 14),
            &mut AtrComponent::new(ctx.clone(), 14),
            &df.test_target(),
        );
    }

    #[test]
    fn length_1_with_atr_length_1() {
        let (df, ctx) = Fixture::load_ctx(&format_path("atr/length_1_with_atr_length_1.csv"));
        _test_with_atr(
            &mut SumComponent::new(ctx.clone(), 1),
            &mut AtrComponent::new(ctx.clone(), 1),
            &df.test_target(),
        );
    }

    #[test]
    fn length_14_with_atr_length_1() {
        let (df, ctx) = Fixture::load_ctx(&format_path("atr/length_14_with_atr_length_1.csv"));
        _test_with_atr(
            &mut SumComponent::new(ctx.clone(), 14),
            &mut AtrComponent::new(ctx.clone(), 1),
            &df.test_target(),
        );
    }
}
