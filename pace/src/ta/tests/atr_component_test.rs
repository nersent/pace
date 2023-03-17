#[cfg(test)]
mod tests {
    use crate::{
        components::component::Component,
        ta::{
            atr_component::AtrComponent, ema_component::EmaComponent, rma_component::RmaComponent,
        },
        testing::{
            array_snapshot::ArraySnapshot,
            fixture::{DataFrameFixtureUtils, Fixture},
        },
    };

    fn format_path(path: &str) -> String {
        format!("tests/ta/atr/{}", path)
    }

    fn _test(target: &mut AtrComponent, expected: &[Option<f64>]) {
        let mut snapshot = ArraySnapshot::<Option<f64>>::new();
        for _ in target.ctx.clone() {
            let output = target.next(());
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn length_1() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_1.csv"));
        _test(&mut AtrComponent::new(ctx.clone(), 1), &df.test_target());
    }

    #[test]
    fn length_2() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_2.csv"));
        _test(&mut AtrComponent::new(ctx.clone(), 2), &df.test_target());
    }

    #[test]
    fn length_14() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_14.csv"));
        _test(&mut AtrComponent::new(ctx.clone(), 14), &df.test_target());
    }
}
