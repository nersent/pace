#[cfg(test)]
mod tests {
    use crate::{
        components::component::Component,
        ta::{ema_component::EmaComponent, roc_component::RocComponent},
        testing::{
            array_snapshot::ArraySnapshot,
            fixture::{DataFrameFixtureUtils, Fixture},
        },
    };

    fn format_path(path: &str) -> String {
        format!("tests/ta/roc/{}", path)
    }

    fn _test(target: &mut RocComponent, expected: &[Option<f64>]) {
        let mut snapshot = ArraySnapshot::<Option<f64>>::new();
        for _ in target.ctx.clone() {
            let ouptut = target.next(target.ctx.close());
            snapshot.push(ouptut);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn length_1_close() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_1_close.csv"));
        _test(&mut RocComponent::new(ctx.clone(), 1), &df.test_target());
    }

    #[test]
    fn length_2_close() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_2_close.csv"));
        _test(&mut RocComponent::new(ctx.clone(), 2), &df.test_target());
    }

    #[test]
    fn length_3_close() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_3_close.csv"));
        _test(&mut RocComponent::new(ctx.clone(), 3), &df.test_target());
    }

    #[test]
    fn length_365_close() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_365_close.csv"));
        _test(&mut RocComponent::new(ctx.clone(), 365), &df.test_target());
    }
}
