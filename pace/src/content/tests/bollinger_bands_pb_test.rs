#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{
        common::src::{Src, SrcKind},
        content::{
            aroon::{Aroon, AroonConfig},
            awesome_oscillator::{AwesomeOscillator, AwesomeOscillatorConfig},
            bollinger_bands_pb::{BollingerBandsPercentB, BollingerBandsPercentBConfig},
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
            "tests/content/bollinger_bands_pb/indicator/{}",
            path
        ))
    }

    fn _test(target: &mut BollingerBandsPercentB, expected: &[f64]) {
        let mut snapshot = ArraySnapshot::<f64>::new();
        for _ in target.ctx.clone() {
            let output = target.next(());
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn length_20_mult_2_close() {
        let (df, ctx) = Fixture::load(&format_path("length_20_sma_mult_2_close.csv"));
        _test(
            &mut BollingerBandsPercentB::new(
                ctx.clone(),
                BollingerBandsPercentBConfig {
                    length: 20,
                    mult: 2.0,
                    src: Src::new(ctx.clone(), SrcKind::Close).to_box(),
                },
            ),
            &df.test_target(),
        );
    }
}
