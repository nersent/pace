#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{
        common::src::{Src, SrcKind},
        content::{
            aroon::{Aroon, AroonConfig},
            awesome_oscillator::{AwesomeOscillator, AwesomeOscillatorConfig},
            relative_vigor_index::{RelativeVigorIndex, RelativeVigorIndexConfig},
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
            "tests/content/relative_vigor_index/indicator/{}",
            path
        ))
    }

    fn _test(target: &mut RelativeVigorIndex, expected: &[(f64, f64)]) {
        let mut snapshot = ArraySnapshot::<(f64, f64)>::new();
        for _ in target.ctx.clone() {
            let output = target.next(());
            snapshot.push((output.rvi, output.sig));
        }
        snapshot.assert(expected);
    }

    #[test]
    fn length_14() {
        let (df, ctx) = Fixture::load(&format_path("length_14.csv"));
        let expected = df.merge_two_columns("_target_rvi_", "_target_sig_");
        _test(
            &mut RelativeVigorIndex::new(ctx.clone(), RelativeVigorIndexConfig { length: 14 }),
            &expected,
        );
    }

    #[test]
    fn length_1() {
        let (df, ctx) = Fixture::load(&format_path("length_1.csv"));
        let expected = df.merge_two_columns("_target_rvi_", "_target_sig_");
        _test(
            &mut RelativeVigorIndex::new(ctx.clone(), RelativeVigorIndexConfig { length: 1 }),
            &expected,
        );
    }

    #[test]
    fn length_2() {
        let (df, ctx) = Fixture::load(&format_path("length_2.csv"));
        let expected = df.merge_two_columns("_target_rvi_", "_target_sig_");
        _test(
            &mut RelativeVigorIndex::new(ctx.clone(), RelativeVigorIndexConfig { length: 2 }),
            &expected,
        );
    }

    #[test]
    fn length_3() {
        let (df, ctx) = Fixture::load(&format_path("length_3.csv"));
        let expected = df.merge_two_columns("_target_rvi_", "_target_sig_");
        _test(
            &mut RelativeVigorIndex::new(ctx.clone(), RelativeVigorIndexConfig { length: 3 }),
            &expected,
        );
    }

    #[test]
    fn length_365() {
        let (df, ctx) = Fixture::load(&format_path("length_365.csv"));
        let expected = df.merge_two_columns("_target_rvi_", "_target_sig_");
        _test(
            &mut RelativeVigorIndex::new(ctx.clone(), RelativeVigorIndexConfig { length: 365 }),
            &expected,
        );
    }
}
