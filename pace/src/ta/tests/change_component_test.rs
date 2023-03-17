#[cfg(test)]
mod tests {
    use crate::{
        components::component::Component,
        ta::{change_component::ChangeComponent, ema_component::EmaComponent},
        testing::{
            array_snapshot::ArraySnapshot,
            fixture::{DataFrameFixtureUtils, Fixture},
        },
    };

    fn format_path(path: &str) -> String {
        format!("tests/ta/change/{}", path)
    }

    fn _test(target: &mut ChangeComponent, expected: &[Option<f64>]) {
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
        _test(&mut ChangeComponent::new(ctx.clone(), 1), &df.test_target());
    }

    #[test]
    fn length_2_close() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_2_close.csv"));
        _test(&mut ChangeComponent::new(ctx.clone(), 2), &df.test_target());
    }

    #[test]
    fn length_3_close() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_3_close.csv"));
        _test(&mut ChangeComponent::new(ctx.clone(), 3), &df.test_target());
    }

    #[test]
    fn length_365_close() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_365_close.csv"));
        _test(
            &mut ChangeComponent::new(ctx.clone(), 365),
            &df.test_target(),
        );
    }
}
