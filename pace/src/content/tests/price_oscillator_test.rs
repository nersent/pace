#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{
        common::src::{Src, SrcKind},
        content::{
            aroon::{Aroon, AroonConfig},
            awesome_oscillator::{AwesomeOscillator, AwesomeOscillatorConfig},
            price_oscillator::{PriceOscillator, PriceOscillatorConfig},
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
            "tests/content/price_oscillator/indicator/{}",
            path
        ))
    }

    fn _test(target: &mut PriceOscillator, expected: &[f64]) {
        let mut snapshot = ArraySnapshot::<f64>::new();
        for _ in target.ctx.clone() {
            let output = target.next(());
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn long_length_21_short_length_10_long_ma_sma_short_ma_sma_close() {
        let (df, ctx) = Fixture::load(&format_path(
            "long_length_21_short_length_10_long_ma_sma_short_ma_sma_close.csv",
        ));
        _test(
            &mut PriceOscillator::new(
                ctx.clone(),
                PriceOscillatorConfig {
                    short_ma: Ma::new(ctx.clone(), MaKind::SMA, 10).to_box(),
                    long_ma: Ma::new(ctx.clone(), MaKind::SMA, 21).to_box(),
                    src: Src::new(ctx.clone(), SrcKind::Close).to_box(),
                },
            ),
            &df.test_target(),
        );
    }
}
