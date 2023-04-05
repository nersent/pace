#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{
        common::src::{Src, SrcKind},
        content::{
            aroon::{Aroon, AroonConfig},
            awesome_oscillator::{AwesomeOscillator, AwesomeOscillatorConfig},
            volume_oscillator::{VolumeOscillator, VolumeOscillatorConfig},
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
            "tests/content/volume_oscillator/indicator/{}",
            path
        ))
    }

    fn _test(target: &mut VolumeOscillator, expected: &[f64]) {
        let mut snapshot = ArraySnapshot::<f64>::new();
        for _ in target.ctx.clone() {
            let output = target.next(());
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn short_length_5_long_length_10_ema() {
        let (df, ctx) = Fixture::load(&format_path("short_length_5_long_length_10_ema.csv"));

        _test(
            &mut VolumeOscillator::new(
                ctx.clone(),
                VolumeOscillatorConfig {
                    short_ma: Ma::new(ctx.clone(), MaKind::EMA, 5).to_box(),
                    long_ma: Ma::new(ctx.clone(), MaKind::EMA, 10).to_box(),
                },
            ),
            &df.test_target(),
        );
    }

    #[test]
    fn short_length_1_long_length_1_ema() {
        let (df, ctx) = Fixture::load(&format_path("short_length_1_long_length_1_ema.csv"));

        _test(
            &mut VolumeOscillator::new(
                ctx.clone(),
                VolumeOscillatorConfig {
                    short_ma: Ma::new(ctx.clone(), MaKind::EMA, 1).to_box(),
                    long_ma: Ma::new(ctx.clone(), MaKind::EMA, 1).to_box(),
                },
            ),
            &df.test_target(),
        );
    }
}
