#[cfg(test)]
mod tests {
    use crate::{
        components::component::Component,
        ta::{ema_component::EmaComponent, rma_component::RmaComponent, tr_component::TrComponent},
        testing::{
            array_snapshot::ArraySnapshot,
            fixture::{DataFrameFixtureUtils, Fixture},
        },
    };

    fn format_path(path: &str) -> String {
        format!("tests/ta/tr/{}", path)
    }

    fn _test(target: &mut TrComponent, expected: &[Option<f64>]) {
        let mut snapshot = ArraySnapshot::<Option<f64>>::new();
        for _ in target.ctx.clone() {
            let output = target.next(());
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn without_handle_na() {
        let (df, ctx) = Fixture::load_ctx(&format_path("without_handle.csv"));
        _test(&mut TrComponent::new(ctx.clone(), false), &df.test_target());
    }

    #[test]
    fn with_handle_na() {
        let (df, ctx) = Fixture::load_ctx(&format_path("with_handle.csv"));
        _test(&mut TrComponent::new(ctx.clone(), true), &df.test_target());
    }
}
