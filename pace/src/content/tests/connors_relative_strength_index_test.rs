#[cfg(test)]
mod tests {
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
        },
    };

    fn format_path(path: &str) -> String {
        format!(
            "tests/content/connors_relative_strength_index/indicator/{}",
            path
        )
    }

    fn _test(target: &mut ConnorsRelativeStrengthIndex, expected: &[Option<f64>]) {
        let mut snapshot = ArraySnapshot::<Option<f64>>::new();
        for _ in target.ctx.clone() {
            let output = target.next(());
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn length_3_up_down_len_2_roc_length_100_close() {
        let (df, ctx) = Fixture::load_ctx(&format_path(
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
