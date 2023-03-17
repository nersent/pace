#[cfg(test)]
mod tests {
    use crate::{
        components::component::Component,
        ta::ema_component::EmaComponent,
        testing::{
            array_snapshot::ArraySnapshot,
            fixture::{DataFrameFixtureUtils, Fixture},
        },
    };

    fn format_path(path: &str) -> String {
        format!("tests/ta/ema/{}", path)
    }

    fn _test(target: &mut EmaComponent, expected: &[Option<f64>]) {
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
        _test(&mut EmaComponent::new(ctx.clone(), 1), &df.test_target());
    }

    #[test]
    fn length_2_close() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_2_close.csv"));
        _test(&mut EmaComponent::new(ctx.clone(), 2), &df.test_target());
    }

    #[test]
    fn length_3_close() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_3_close.csv"));
        _test(&mut EmaComponent::new(ctx.clone(), 3), &df.test_target());
    }

    #[test]
    fn length_7_close() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_7_close.csv"));
        _test(&mut EmaComponent::new(ctx.clone(), 7), &df.test_target());
    }

    #[test]
    fn length_14_close() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_14_close.csv"));
        _test(&mut EmaComponent::new(ctx.clone(), 14), &df.test_target());
    }

    #[test]
    fn length_350_close() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_350_close.csv"));
        _test(&mut EmaComponent::new(ctx.clone(), 350), &df.test_target());
    }
}
