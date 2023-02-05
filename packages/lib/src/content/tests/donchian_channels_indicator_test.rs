#[cfg(test)]
mod tests {
    use crate::{
        base::components::{
            component_context::ComponentContext,
            testing::{ComponentTestSnapshot, Fixture},
        },
        content::donchian_channels_indicator::{
            DonchianChannelsIndicator, DonchianChannelsIndicatorConfig,
        },
        utils::polars::DataFrameUtils,
    };

    fn format_path(path: &str) -> String {
        format!(
            "content/tests/fixtures/donchian_channels/indicator/{}",
            path
        )
    }

    fn _test(
        cctx: &mut ComponentContext,
        target: &mut DonchianChannelsIndicator,
        expected: &[Option<(Option<f64>, Option<f64>, Option<f64>)>],
    ) {
        let mut snapshot = ComponentTestSnapshot::<(Option<f64>, Option<f64>, Option<f64>)>::new();
        for cctx in cctx {
            let output = target.next();
            snapshot.push(Some((output.upper, output.basis, output.lower)));
        }
        snapshot.assert(expected);
    }

    #[test]
    fn length_14() {
        let (_df, ctx) = Fixture::raw(&format_path("length_14.csv"));
        let expected =
            _df.merge_three_columns("_target_upper_", "_target_basis_", "_target_lower_");
        _test(
            &mut ctx.clone(),
            &mut DonchianChannelsIndicator::new(
                ctx.clone(),
                DonchianChannelsIndicatorConfig { length: 14 },
            ),
            &expected,
        );
    }
}
