#[cfg(test)]
mod tests {
    use crate::{
        base::components::{
            component_context::ComponentContext,
            testing::{ComponentTestSnapshot, Fixture},
        },
        content::chande_kroll_stop_indicator::{
            ChandeKrollStopIndicator, ChandeKrollStopIndicatorConfig,
        },
        utils::polars::{DataFrameUtils, SeriesCastUtils},
    };

    fn format_path(path: &str) -> String {
        format!(
            "content/tests/fixtures/chande_kroll_stop/indicator/{}",
            path
        )
    }

    fn _test(
        cctx: &mut ComponentContext,
        target: &mut ChandeKrollStopIndicator,
        expected: &[Option<(Option<f64>, Option<f64>, Option<f64>, Option<f64>)>],
    ) {
        let mut snapshot =
            ComponentTestSnapshot::<(Option<f64>, Option<f64>, Option<f64>, Option<f64>)>::new();
        for cctx in cctx {
            let output = target.next();
            snapshot.push(Some((
                output.first_high_stop,
                output.first_low_stop,
                output.stop_short,
                output.stop_long,
            )));
        }
        snapshot.assert(expected);
    }

    #[test]
    fn p_10_x_1_q_9() {
        let (_df, ctx) = Fixture::raw(&format_path("p_10_x_1_q_9.csv"));
        let expected = _df.merge_four_columns(
            "_target_first_high_stop_",
            "_target_first_low_stop_",
            "_target_stop_short_",
            "_target_stop_long_",
        );
        _test(
            &mut ctx.clone(),
            &mut ChandeKrollStopIndicator::new(
                ctx.clone(),
                ChandeKrollStopIndicatorConfig {
                    p: 10,
                    x: 1.0,
                    q: 9,
                },
            ),
            &expected,
        );
    }
}
