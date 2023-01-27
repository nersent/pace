#[cfg(test)]
mod tests {
    use crate::{
        components::{component_context::ComponentContext, testing::ComponentTestSnapshot},
        data::polars::SeriesCastUtils,
        ta::aroon::aroon_indicator::{AroonIndicator, AroonIndicatorConfig},
        testing::fixture::Fixture,
    };

    fn _test(
        cctx: &mut ComponentContext,
        target: &mut AroonIndicator,
        expected: &[Option<(Option<f64>, Option<f64>)>],
    ) {
        let mut snapshot = ComponentTestSnapshot::<(Option<f64>, Option<f64>)>::new();
        for cctx in cctx {
            let output = target.next();
            snapshot.push(Some((output.up, output.down)));
        }
        snapshot.assert(expected);
    }

    #[test]
    fn test_aroon_btc_1d_length_14() {
        let (_df, ctx) = Fixture::raw("ta/aroon/tests/fixtures/indicator/btc_1d_length_14.csv");
        let up_values = _df.column("_target_up_").unwrap().to_f64();
        let down_values = _df.column("_target_down_").unwrap().to_f64();
        let expected: Vec<Option<(Option<f64>, Option<f64>)>> = up_values
            .iter()
            .zip(down_values.iter())
            .map(|(up, down)| Some((*up, *down)))
            .collect();
        _test(
            &mut ctx.clone(),
            &mut AroonIndicator::new(ctx.clone(), AroonIndicatorConfig { length: 14 }),
            &expected,
        );
    }
}
