#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{
        common::src::{Src, SrcKind},
        content::{
            aroon::{Aroon, AroonConfig},
            awesome_oscillator::{AwesomeOscillator, AwesomeOscillatorConfig},
            connors_relative_strength_index::{
                ConnorsRelativeStrengthIndex, ConnorsRelativeStrengthIndexConfig,
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
            "tests/content/connors_relative_strength_index/indicator/{}",
            path
        ))
    }

    fn _test(target: &mut ConnorsRelativeStrengthIndex, expected: &[f64]) {
        let mut snapshot = ArraySnapshot::<f64>::new();
        for _ in target.ctx.clone() {
            let output = target.next(());
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn length_3_up_down_len_2_roc_length_100_close() {
        let (df, ctx) = Fixture::load(&format_path(
            "length_3_up_down_len_2_roc_length_100_close.csv",
        ));

        _test(
            &mut ConnorsRelativeStrengthIndex::new(
                ctx.clone(),
                ConnorsRelativeStrengthIndexConfig {
                    length_rsi: 3,
                    length_up_down: 2,
                    length_roc: 100,
                    src: Src::new(ctx.clone(), SrcKind::Close).to_box(),
                },
            ),
            &df.test_target(),
        );
    }
}
