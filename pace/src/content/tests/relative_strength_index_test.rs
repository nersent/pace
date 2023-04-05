#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{
        common::src::{Src, SrcKind},
        content::{
            aroon::{Aroon, AroonConfig},
            awesome_oscillator::{AwesomeOscillator, AwesomeOscillatorConfig},
            relative_strength_index::{
                RelativeStrengthIndex, RelativeStrengthIndexConfig, RelativeStrengthIndexStrategy,
                RelativeStrengthIndexStrategyConfig, RELATIVE_STRENGTH_INDEX_THRESHOLD_OVERBOUGHT,
                RELATIVE_STRENGTH_INDEX_THRESHOLD_OVERSOLD,
            },
        },
        core::incremental::Incremental,
        polars::dataframe::DataFrameUtils,
        strategy::trade::{StrategySignal, TradeDirection},
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

    fn format_indicator_path(path: &str) -> PathBuf {
        format_pace_fixture_path(&format!(
            "tests/content/relative_strength_index/indicator/{}",
            path
        ))
    }

    fn _test_indicator(target: &mut RelativeStrengthIndex, expected: &[f64]) {
        let mut snapshot = ArraySnapshot::<f64>::new();
        for _ in target.ctx.clone() {
            let output = target.next(());
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn indicator_length_14_open() {
        let (df, ctx) = Fixture::load(&format_indicator_path("length_14_open.csv"));
        _test_indicator(
            &mut RelativeStrengthIndex::new(
                ctx.clone(),
                RelativeStrengthIndexConfig {
                    length: 14,
                    src: Src::new(ctx.clone(), SrcKind::Open).to_box(),
                },
            ),
            &df.test_target(),
        );
    }

    fn format_strategy_path(path: &str) -> PathBuf {
        format_pace_fixture_path(&format!(
            "tests/content/relative_strength_index/strategy/{}",
            path
        ))
    }

    fn _test_strategy(
        target: &mut RelativeStrengthIndexStrategy,
        target_indicator: &mut RelativeStrengthIndex,
        expected: &[f64],
    ) {
        let mut snapshot = ArraySnapshot::<f64>::new();
        for _ in target.ctx.clone() {
            let output_indicator = target_indicator.next(());
            let output = target.next(output_indicator);
            snapshot.push(output.into());
        }
        snapshot.assert(expected);
    }

    #[test]
    fn strategy_length_14_close() {
        let (df, ctx) = Fixture::load(&format_strategy_path("length_14_close.csv"));
        _test_strategy(
            &mut RelativeStrengthIndexStrategy::new(
                ctx.clone(),
                RelativeStrengthIndexStrategyConfig {
                    threshold_oversold: RELATIVE_STRENGTH_INDEX_THRESHOLD_OVERSOLD,
                    threshold_overbought: RELATIVE_STRENGTH_INDEX_THRESHOLD_OVERBOUGHT,
                },
            ),
            &mut RelativeStrengthIndex::new(
                ctx.clone(),
                RelativeStrengthIndexConfig {
                    length: 14,
                    src: Src::new(ctx.clone(), SrcKind::Close).to_box(),
                },
            ),
            &df.test_target(),
        );
    }
}
