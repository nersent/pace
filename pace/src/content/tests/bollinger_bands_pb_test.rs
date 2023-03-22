#[cfg(test)]
mod tests {
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
        },
    };

    fn format_path(path: &str) -> String {
        format!("tests/content/bollinger_bands_pb/indicator/{}", path)
    }

    fn _test(target: &mut BollingerBandsPercentB, expected: &[Option<f64>]) {
        let mut snapshot = ArraySnapshot::<Option<f64>>::new();
        for _ in target.ctx.clone() {
            let output = target.next(());
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn length_20_mult_2_close() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_20_sma_mult_2_close.csv"));
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
