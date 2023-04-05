#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{
        common::src::{Src, SrcKind},
        content::{
            aroon::{Aroon, AroonConfig},
            awesome_oscillator::{AwesomeOscillator, AwesomeOscillatorConfig},
            stoch_relative_strength_index::{
                StochRelativeStrengthIndex, StochRelativeStrengthIndexConfig,
            },
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
            "tests/content/stoch_relative_strength_index/indicator/{}",
            path
        ))
    }

    fn _test(target: &mut StochRelativeStrengthIndex, expected: &[(f64, f64)]) {
        let mut snapshot = ArraySnapshot::<(f64, f64)>::new();
        for _ in target.ctx.clone() {
            let output = target.next(());
            snapshot.push((output.k, output.d));
        }
        snapshot.assert(expected);
    }

    #[test]
    fn length_14_stoch_length_14_k_3_d_3_close() {
        let (df, ctx) = Fixture::load(&format_path("length_14_stoch_length_14_k_3_d_3_close.csv"));
        let expected = df.merge_two_columns("_target_k_", "_target_d_");
        _test(
            &mut StochRelativeStrengthIndex::new(
                ctx.clone(),
                StochRelativeStrengthIndexConfig {
                    length_rsi: 14,
                    length_stoch: 14,
                    smooth_d: 3,
                    smooth_k: 3,
                    src: Src::new(ctx.clone(), SrcKind::Close).to_box(),
                },
            ),
            &expected,
        );
    }

    #[test]
    fn length_2_stoch_length_2_k_3_d_3_close() {
        let (df, ctx) = Fixture::load(&format_path("length_2_stoch_length_2_k_3_d_3_close.csv"));
        let expected = df.merge_two_columns("_target_k_", "_target_d_");
        _test(
            &mut StochRelativeStrengthIndex::new(
                ctx.clone(),
                StochRelativeStrengthIndexConfig {
                    length_rsi: 2,
                    length_stoch: 2,
                    smooth_d: 3,
                    smooth_k: 3,
                    src: Src::new(ctx.clone(), SrcKind::Close).to_box(),
                },
            ),
            &expected,
        );
    }

    #[test]
    fn length_2_stoch_length_2_k_14_d_14_close() {
        let (df, ctx) = Fixture::load(&format_path("length_2_stoch_length_2_k_14_d_14_close.csv"));
        let expected = df.merge_two_columns("_target_k_", "_target_d_");
        _test(
            &mut StochRelativeStrengthIndex::new(
                ctx.clone(),
                StochRelativeStrengthIndexConfig {
                    length_rsi: 2,
                    length_stoch: 2,
                    smooth_d: 14,
                    smooth_k: 14,
                    src: Src::new(ctx.clone(), SrcKind::Close).to_box(),
                },
            ),
            &expected,
        );
    }
}
