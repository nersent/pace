#[cfg(test)]
mod tests {
    use crate::{
        components::component::Component,
        ta::{highest_bars_component::HighestBarsComponent, stoch_component::StochComponent},
        testing::{
            array_snapshot::ArraySnapshot,
            fixture::{DataFrameFixtureUtils, Fixture},
        },
    };

    fn format_path(path: &str) -> String {
        format!("tests/ta/stoch/{}", path)
    }

    fn _test_close_high_low(target: &mut StochComponent, expected: &[Option<f64>]) {
        let mut snapshot = ArraySnapshot::<Option<f64>>::new();
        for _ in target.ctx.clone() {
            let ouptut = target.next((target.ctx.close(), target.ctx.high(), target.ctx.low()));
            snapshot.push(ouptut);
        }
        snapshot.assert(expected);
    }

    fn _test_close_close_close(target: &mut StochComponent, expected: &[Option<f64>]) {
        let mut snapshot = ArraySnapshot::<Option<f64>>::new();
        for _ in target.ctx.clone() {
            let ouptut = target.next((target.ctx.close(), target.ctx.close(), target.ctx.close()));
            snapshot.push(ouptut);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn length_14_close_high_low() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_14_close_high_low.csv"));
        _test_close_high_low(&mut StochComponent::new(ctx.clone(), 14), &df.test_target());
    }

    #[test]
    fn length_1_close_high_low() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_1_close_high_low.csv"));
        _test_close_high_low(&mut StochComponent::new(ctx.clone(), 1), &df.test_target());
    }

    #[test]
    fn length_2_close_high_low() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_2_close_high_low.csv"));
        _test_close_high_low(&mut StochComponent::new(ctx.clone(), 2), &df.test_target());
    }

    #[test]
    fn length_3_close_high_low() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_3_close_high_low.csv"));
        _test_close_high_low(&mut StochComponent::new(ctx.clone(), 3), &df.test_target());
    }

    #[test]
    fn length_1_close_close_close() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_1_close_close_close.csv"));
        _test_close_close_close(&mut StochComponent::new(ctx.clone(), 1), &df.test_target());
    }

    #[test]
    fn length_2_close_close_close() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_2_close_close_close.csv"));
        _test_close_close_close(&mut StochComponent::new(ctx.clone(), 2), &df.test_target());
    }

    #[test]
    fn length_3_close_close_close() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_3_close_close_close.csv"));
        _test_close_close_close(&mut StochComponent::new(ctx.clone(), 3), &df.test_target());
    }

    #[test]
    fn length_14_close_close_close() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_14_close_close_close.csv"));
        _test_close_close_close(&mut StochComponent::new(ctx.clone(), 14), &df.test_target());
    }
}
