#[cfg(test)]
mod tests {
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
        },
    };

    fn format_path(path: &str) -> String {
        format!("tests/content/macd/indicator/{}", path)
    }

    fn _test(target: &mut Macd, expected: &[Option<f64>]) {
        let mut snapshot = ArraySnapshot::<Option<f64>>::new();
        for _ in target.ctx.clone() {
            let (macd, signal) = target.next(());
            snapshot.push(macd);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn short_length_12_long_length_26_ema_close() {
        let (df, ctx) =
            Fixture::load_ctx(&format_path("short_length_12_long_length_26_ema_close.csv"));
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
