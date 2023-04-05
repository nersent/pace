#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{
        common::src::{Src, SrcKind},
        content::{
            aroon::{Aroon, AroonConfig},
            awesome_oscillator::{AwesomeOscillator, AwesomeOscillatorConfig},
            ultimate_oscillator::{UltimateOscillator, UltimateOscillatorConfig},
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
            "tests/content/ultimate_oscillator/indicator/{}",
            path
        ))
    }

    fn _test(target: &mut UltimateOscillator, expected: &[f64]) {
        let mut snapshot = ArraySnapshot::<f64>::new();
        for _ in target.ctx.clone() {
            let output = target.next(());
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn short_length_7_mid_length_14_long_length_28() {
        let (df, ctx) = Fixture::load(&format_path(
            "short_length_7_mid_length_14_long_length_28.csv",
        ));

        _test(
            &mut UltimateOscillator::new(
                ctx.clone(),
                UltimateOscillatorConfig {
                    short_length: 7,
                    mid_length: 14,
                    long_length: 28,
                },
            ),
            &df.test_target(),
        );
    }

    #[test]
    fn short_length_1_mid_length_1_long_length_1() {
        let (df, ctx) = Fixture::load(&format_path(
            "short_length_1_mid_length_1_long_length_1.csv",
        ));

        _test(
            &mut UltimateOscillator::new(
                ctx.clone(),
                UltimateOscillatorConfig {
                    short_length: 1,
                    mid_length: 1,
                    long_length: 1,
                },
            ),
            &df.test_target(),
        );
    }

    #[test]
    fn short_length_30_mid_length_15_long_length_7() {
        let (df, ctx) = Fixture::load(&format_path(
            "short_length_30_mid_length_15_long_length_7.csv",
        ));

        _test(
            &mut UltimateOscillator::new(
                ctx.clone(),
                UltimateOscillatorConfig {
                    short_length: 30,
                    mid_length: 15,
                    long_length: 7,
                },
            ),
            &df.test_target(),
        );
    }
}
