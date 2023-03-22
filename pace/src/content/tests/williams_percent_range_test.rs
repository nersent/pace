#[cfg(test)]
mod tests {
    use crate::{
        common::src::{Src, SrcKind},
        content::{
            aroon::{Aroon, AroonConfig},
            awesome_oscillator::{AwesomeOscillator, AwesomeOscillatorConfig},
            williams_percent_range::{WilliamsPercentRank, WilliamsPercentRankConfig},
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
        format!("tests/content/williams_percent_range/indicator/{}", path)
    }

    fn _test(target: &mut WilliamsPercentRank, expected: &[Option<f64>]) {
        let mut snapshot = ArraySnapshot::<Option<f64>>::new();
        for _ in target.ctx.clone() {
            let output = target.next(());
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn length_14_close() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_14_close.csv"));
        _test(
            &mut WilliamsPercentRank::new(
                ctx.clone(),
                WilliamsPercentRankConfig {
                    length: 14,
                    src: Src::new(ctx.clone(), SrcKind::Close).to_box(),
                },
            ),
            &df.test_target(),
        );
    }

    #[test]
    fn length_1_close() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_1_close.csv"));
        _test(
            &mut WilliamsPercentRank::new(
                ctx.clone(),
                WilliamsPercentRankConfig {
                    length: 1,
                    src: Src::new(ctx.clone(), SrcKind::Close).to_box(),
                },
            ),
            &df.test_target(),
        );
    }

    #[test]
    fn length_2_close() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_2_close.csv"));
        _test(
            &mut WilliamsPercentRank::new(
                ctx.clone(),
                WilliamsPercentRankConfig {
                    length: 2,
                    src: Src::new(ctx.clone(), SrcKind::Close).to_box(),
                },
            ),
            &df.test_target(),
        );
    }
}
