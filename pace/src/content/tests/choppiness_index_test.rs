#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{
        common::src::{Src, SrcKind},
        content::{
            aroon::{Aroon, AroonConfig},
            awesome_oscillator::{AwesomeOscillator, AwesomeOscillatorConfig},
            choppiness_index::{ChoppinessIndex, ChoppinessIndexConfig},
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
            "tests/content/choppiness_index/indicator/{}",
            path
        ))
    }

    fn _test(target: &mut ChoppinessIndex, expected: &[f64]) {
        let mut snapshot = ArraySnapshot::<f64>::new();
        for _ in target.ctx.clone() {
            let output = target.next(());
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn length_14() {
        let (df, ctx) = Fixture::load(&format_path("length_14.csv"));
        _test(
            &mut ChoppinessIndex::new(ctx.clone(), ChoppinessIndexConfig { length: 14 }),
            &df.test_target(),
        );
    }

    #[test]
    fn length_2() {
        let (df, ctx) = Fixture::load(&format_path("length_2.csv"));
        _test(
            &mut ChoppinessIndex::new(ctx.clone(), ChoppinessIndexConfig { length: 2 }),
            &df.test_target(),
        );
    }
}
