#[cfg(test)]
mod tests {
    use crate::{
        components::component::Component,
        ta::{
            highest_bars_component::HighestBarsComponent,
            lowest_bars_component::LowestBarsComponent,
        },
        testing::{
            array_snapshot::ArraySnapshot,
            fixture::{DataFrameFixtureUtils, Fixture},
        },
    };

    fn format_path(path: &str) -> String {
        format!("tests/ta/bars/lowest_bars/{}", path)
    }

    fn _test(target: &mut LowestBarsComponent, expected: &[Option<f64>]) {
        let mut snapshot = ArraySnapshot::<Option<f64>>::new();
        for _ in target.ctx.clone() {
            let output = target.next(());
            snapshot.push(output.map(|x| x as f64));
        }
        snapshot.assert(expected);
    }

    #[test]
    fn length_14_high() {
        let (_df, ctx) = Fixture::load_ctx(&format_path("length_14.csv"));
        _test(
            &mut LowestBarsComponent::new(ctx.clone(), 14),
            &_df.test_target(),
        );
    }
}