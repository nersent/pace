#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{
        common::src::{Src, SrcKind},
        content::{
            aroon::{Aroon, AroonConfig},
            awesome_oscillator::{AwesomeOscillator, AwesomeOscillatorConfig},
            chande_kroll_stop::{ChandeKrollStop, ChandeKrollStopConfig},
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
            pace::format_pace_fixture_path,
        },
    };

    fn format_path(path: &str) -> PathBuf {
        format_pace_fixture_path(&format!(
            "tests/content/chande_kroll_stop/indicator/{}",
            path
        ))
    }

    fn _test(target: &mut ChandeKrollStop, expected: &[(f64, f64, f64, f64)]) {
        let mut snapshot = ArraySnapshot::<(f64, f64, f64, f64)>::new();
        for _ in target.ctx.clone() {
            let output = target.next(());
            snapshot.push((
                output.first_high_stop,
                output.first_low_stop,
                output.stop_short,
                output.stop_long,
            ));
        }
        snapshot.assert(expected);
    }

    #[test]
    fn p_10_x_1_q_9() {
        let (df, ctx) = Fixture::load(&format_path("p_10_x_1_q_9.csv"));
        let expected = df.merge_four_columns(
            "_target_first_high_stop_",
            "_target_first_low_stop_",
            "_target_stop_short_",
            "_target_stop_long_",
        );
        _test(
            &mut ChandeKrollStop::new(
                ctx.clone(),
                ChandeKrollStopConfig {
                    p: 10,
                    x: 1.0,
                    q: 9,
                },
            ),
            &expected,
        );
    }
}
