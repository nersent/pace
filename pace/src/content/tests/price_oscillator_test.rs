#[cfg(test)]
mod tests {
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
        },
    };

    fn format_path(path: &str) -> String {
        format!("tests/content/price_oscillator/indicator/{}", path)
    }

    fn _test(target: &mut PriceOscillator, expected: &[Option<f64>]) {
        let mut snapshot = ArraySnapshot::<Option<f64>>::new();
        for _ in target.ctx.clone() {
            let output = target.next(());
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn long_length_21_short_length_10_long_ma_sma_short_ma_sma_close() {
        let (df, ctx) = Fixture::load_ctx(&format_path(
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
