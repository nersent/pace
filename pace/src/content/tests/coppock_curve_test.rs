#[cfg(test)]
mod tests {
    use crate::{
        common::src::{Src, SrcKind},
        content::{
            aroon::{Aroon, AroonConfig},
            awesome_oscillator::{AwesomeOscillator, AwesomeOscillatorConfig},
            coppock_curve::{CoppockCurve, CoppockCurveConfig},
        },
        core::incremental::Incremental,
        polars::dataframe::DataFrameUtils,
        ta::{
            moving_average::{Ma, MaKind},
            simple_moving_average::Sma,
        },
        testing::{
            array_snapshot::ArraySnapshot,
            fixture::{DataFrameFixtureUtils, Fixture},
        },
    };

    fn format_path(path: &str) -> String {
        format!("tests/content/coppock_curve/indicator/{}", path)
    }

    fn _test(target: &mut CoppockCurve, expected: &[Option<f64>]) {
        let mut snapshot = ArraySnapshot::<Option<f64>>::new();
        for _ in target.ctx.clone() {
            let output = target.next(());
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn long_roc_length_14_short_roc_length_11_ma_length_10_close() {
        let (df, ctx) = Fixture::load_ctx(&format_path(
            "long_roc_length_14_short_roc_length_11_ma_length_10_close.csv",
        ));

        _test(
            &mut CoppockCurve::new(
                ctx.clone(),
                CoppockCurveConfig {
                    length: 10,
                    long_roc_length: 14,
                    short_roc_length: 11,
                    src: Src::new(ctx.clone(), SrcKind::Close).to_box(),
                },
            ),
            &df.test_target(),
        );
    }
}
