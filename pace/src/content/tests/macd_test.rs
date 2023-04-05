#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{
        common::src::{Src, SrcKind},
        content::{
            aroon::{Aroon, AroonConfig},
            awesome_oscillator::{AwesomeOscillator, AwesomeOscillatorConfig},
            macd::{Macd, MacdConfig},
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
        format_pace_fixture_path(&format!("tests/content/macd/indicator/{}", path))
    }

    fn _test(target: &mut Macd, expected: &[f64]) {
        let mut snapshot = ArraySnapshot::<f64>::new();
        for _ in target.ctx.clone() {
            let (macd, signal) = target.next(());
            snapshot.push(macd);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn short_length_12_long_length_26_ema_close() {
        let (df, ctx) = Fixture::load(&format_path("short_length_12_long_length_26_ema_close.csv"));
        _test(
            &mut Macd::new(
                ctx.clone(),
                MacdConfig {
                    short_ma: Ma::new(ctx.clone(), MaKind::EMA, 12).to_box(),
                    long_ma: Ma::new(ctx.clone(), MaKind::EMA, 26).to_box(),
                    short_src: Src::new(ctx.clone(), SrcKind::Close).to_box(),
                    long_src: Src::new(ctx.clone(), SrcKind::Close).to_box(),
                    signal_ma: Ma::new(ctx.clone(), MaKind::EMA, 9).to_box(),
                },
            ),
            &df.test_target(),
        );
    }
}
