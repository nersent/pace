#[cfg(test)]
mod tests {
    use crate::{
        components::component::Component,
        content::chande_kroll_stop_indicator::{CksIndicator, CksIndicatorConfig},
        testing::{array_snapshot::ArraySnapshot, fixture::Fixture},
        utils::polars::DataFrameUtils,
    };

    fn format_path(path: &str) -> String {
        format!("tests/content/chande_kroll_stop/indicator/{}", path)
    }

    fn _test(
        target: &mut CksIndicator,
        expected: &[Option<(Option<f64>, Option<f64>, Option<f64>, Option<f64>)>],
    ) {
        let mut snapshot =
            ArraySnapshot::<Option<(Option<f64>, Option<f64>, Option<f64>, Option<f64>)>>::new();
        for _ in target.ctx.clone() {
            let output = target.next(());
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
        let (df, ctx) = Fixture::load_ctx(&format_path("p_10_x_1_q_9.csv"));
        let expected = df.merge_four_columns(
            "_target_first_high_stop_",
            "_target_first_low_stop_",
            "_target_stop_short_",
            "_target_stop_long_",
        );
        _test(
            &mut CksIndicator::new(
                ctx.clone(),
                CksIndicatorConfig {
                    p: 10,
                    x: 1.0,
                    q: 9,
                },
            ),
            &expected,
        );
    }
}
