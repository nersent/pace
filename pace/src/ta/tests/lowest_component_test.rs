#[cfg(test)]
mod tests {
    use crate::{
        components::component::Component,
        ta::{highest_bars_component::HighestBarsComponent, lowest_component::LowestComponent},
        testing::{
            array_snapshot::ArraySnapshot,
            fixture::{DataFrameFixtureUtils, Fixture},
        },
    };

    fn format_path(path: &str) -> String {
        format!("tests/ta/bars/lowest/{}", path)
    }

    fn _test(target: &mut LowestComponent, expected: &[Option<f64>]) {
        let mut snapshot = ArraySnapshot::<Option<f64>>::new();
        for _ in target.ctx.clone() {
            let output = target.next(target.ctx.low());
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn length_14_low() {
        let (_df, ctx) = Fixture::load_ctx(&format_path("length_14_low.csv"));
        _test(
            &mut LowestComponent::new(ctx.clone(), 14),
            &_df.test_target(),
        );
    }
}
