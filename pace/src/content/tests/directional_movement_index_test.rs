#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{
        common::src::{Src, SrcKind},
        content::{
            aroon::{Aroon, AroonConfig},
            awesome_oscillator::{AwesomeOscillator, AwesomeOscillatorConfig},
            directional_movement_index::{
                DirectionalMovementIndex, DirectionalMovementIndexConfig,
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
            "tests/content/directional_movement_index/indicator/{}",
            path
        ))
    }

    fn _test(target: &mut DirectionalMovementIndex, expected: &[(f64, f64, f64)]) {
        let mut snapshot = ArraySnapshot::<(f64, f64, f64)>::new();
        for _ in target.ctx.clone() {
            let output = target.next(());
            snapshot.push((output.plus, output.minus, output.adx));
        }
        snapshot.assert(expected);
    }

    #[test]
    fn length_14_lensig_14() {
        let (df, ctx) = Fixture::load(&format_path("length_14_lensig_14.csv"));
        let expected = df.merge_three_columns("_target_plus_", "_target_minus_", "_target_adx_");
        _test(
            &mut DirectionalMovementIndex::new(
                ctx.clone(),
                DirectionalMovementIndexConfig {
                    length: 14,
                    lensig: 14,
                },
            ),
            &expected,
        );
    }

    #[test]
    fn length_3_lensig_3() {
        let (df, ctx) = Fixture::load(&format_path("length_3_lensig_3.csv"));
        let expected = df.merge_three_columns("_target_plus_", "_target_minus_", "_target_adx_");
        _test(
            &mut DirectionalMovementIndex::new(
                ctx.clone(),
                DirectionalMovementIndexConfig {
                    length: 3,
                    lensig: 3,
                },
            ),
            &expected,
        );
    }

    #[test]
    fn length_14_lensig_3() {
        let (df, ctx) = Fixture::load(&format_path("length_14_lensig_3.csv"));
        let expected = df.merge_three_columns("_target_plus_", "_target_minus_", "_target_adx_");
        _test(
            &mut DirectionalMovementIndex::new(
                ctx.clone(),
                DirectionalMovementIndexConfig {
                    length: 14,
                    lensig: 3,
                },
            ),
            &expected,
        );
    }

    #[test]
    fn length_3_lensig_14() {
        let (df, ctx) = Fixture::load(&format_path("length_3_lensig_14.csv"));
        let expected = df.merge_three_columns("_target_plus_", "_target_minus_", "_target_adx_");
        _test(
            &mut DirectionalMovementIndex::new(
                ctx.clone(),
                DirectionalMovementIndexConfig {
                    length: 3,
                    lensig: 14,
                },
            ),
            &expected,
        );
    }
}
