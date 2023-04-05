#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{
        common::src::{Src, SrcKind},
        content::{
            aroon::{Aroon, AroonConfig},
            awesome_oscillator::{AwesomeOscillator, AwesomeOscillatorConfig},
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
            "tests/content/awesome_oscillator/indicator/{}",
            path
        ))
    }

    fn _test(target: &mut AwesomeOscillator, expected: &[f64]) {
        let mut snapshot = ArraySnapshot::<f64>::new();
        for _ in target.ctx.clone() {
            let output = target.next(());
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn short_length_5_long_length_34_hl2() {
        let (df, ctx) = Fixture::load(&format_path("short_length_5_long_length_34_hl2.csv"));
        _test(
            &mut AwesomeOscillator::new(
                ctx.clone(),
                AwesomeOscillatorConfig {
                    long_ma: Ma::new(ctx.clone(), MaKind::SMA, 34).to_box(),
                    short_ma: Ma::new(ctx.clone(), MaKind::SMA, 5).to_box(),
                    long_src: Src::new(ctx.clone(), SrcKind::HL2).to_box(),
                    short_src: Src::new(ctx.clone(), SrcKind::HL2).to_box(),
                },
            ),
            &df.test_target(),
        );
    }
}
