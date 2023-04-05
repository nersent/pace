#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{
        common::src::{Src, SrcKind},
        content::{
            aroon::{Aroon, AroonConfig},
            awesome_oscillator::{AwesomeOscillator, AwesomeOscillatorConfig},
            relative_volatility_index::{RelativeVolatilityIndex, RelativeVolatilityIndexConfig},
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
            "tests/content/relative_volatility_index/indicator/{}",
            path
        ))
    }

    fn _test(target: &mut RelativeVolatilityIndex, expected: &[f64]) {
        let mut snapshot = ArraySnapshot::<f64>::new();
        let ctx = target.ctx.clone();
        for _ in target.ctx.clone() {
            let tick = ctx.bar.index();
            let output = target.next(());
            // We need to omit first 250 bars, because of ta.change and NaNs
            if tick < 250 {
                snapshot.push(expected[tick]);
            } else {
                snapshot.push(output);
            }
        }
        snapshot.assert(expected);
    }

    #[test]
    fn length_14_ma_14_ema_close() {
        let (df, ctx) = Fixture::load(&format_path("length_14_ma_14_ema_close.csv"));
        _test(
            &mut RelativeVolatilityIndex::new(
                ctx.clone(),
                RelativeVolatilityIndexConfig {
                    length: 14,
                    ma_length: 14,
                    src: Src::new(ctx.clone(), SrcKind::Close).to_box(),
                },
            ),
            &df.test_target(),
        );
    }
}
