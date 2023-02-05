#[cfg(test)]
mod tests {
    use crate::{
        base::components::{
            component_context::ComponentContext,
            testing::{ComponentTestSnapshot, Fixture},
        },
        content::aroon_indicator::{AroonIndicator, AroonIndicatorConfig},
        utils::polars::{DataFrameUtils, SeriesCastUtils},
    };

    fn format_path(path: &str) -> String {
        format!("content/tests/fixtures/aroon/indicator/{}", path)
    }

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
    fn length_14() {
        let (_df, ctx) = Fixture::raw(&format_path("length_14.csv"));
        let expected = _df.merge_two_columns("_target_up_", "_target_down_");
        _test(
            &mut ctx.clone(),
            &mut AroonIndicator::new(ctx.clone(), AroonIndicatorConfig { length: 14 }),
            &expected,
        );
    }
}
