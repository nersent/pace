#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{
        common::src::{Src, SrcKind},
        content::{
            aroon::{Aroon, AroonConfig},
            awesome_oscillator::{AwesomeOscillator, AwesomeOscillatorConfig},
            vortex::{Vortex, VortexConfig},
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
        format_pace_fixture_path(&format!("tests/content/vortex/indicator/{}", path))
    }

    fn _test(target: &mut Vortex, expected: &[(f64, f64)]) {
        let mut snapshot = ArraySnapshot::<(f64, f64)>::new();
        for _ in target.ctx.clone() {
            let output = target.next(());
            snapshot.push((output.plus, output.minus));
        }
        snapshot.assert(expected);
    }

    #[test]
    fn length_14() {
        let (df, ctx) = Fixture::load(&format_path("length_14.csv"));
        let expected = df.merge_two_columns("_target_plus_", "_target_minus_");
        _test(
            &mut Vortex::new(ctx.clone(), VortexConfig { length: 14 }),
            &expected,
        );
    }

    #[test]
    fn length_2() {
        let (df, ctx) = Fixture::load(&format_path("length_2.csv"));
        let expected = df.merge_two_columns("_target_plus_", "_target_minus_");
        _test(
            &mut Vortex::new(ctx.clone(), VortexConfig { length: 2 }),
            &expected,
        );
    }
}
