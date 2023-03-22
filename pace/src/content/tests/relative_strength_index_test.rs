#[cfg(test)]
mod tests {
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
        strategy::trade::TradeDirection,
        ta::{
            moving_average::{Ma, MaKind},
            simple_moving_average::Sma,
        },
        testing::{
            array_snapshot::ArraySnapshot,
            fixture::{DataFrameFixtureUtils, Fixture},
        },
    };

    fn format_indicator_path(path: &str) -> String {
        format!("tests/content/relative_strength_index/indicator/{}", path)
    }

    fn _test_indicator(target: &mut RelativeStrengthIndex, expected: &[Option<f64>]) {
        let mut snapshot = ArraySnapshot::<Option<f64>>::new();
        for _ in target.ctx.clone() {
            let output = target.next(());
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn indicator_length_14_open() {
        let (df, ctx) = Fixture::load_ctx(&format_indicator_path("length_14_open.csv"));
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

    fn format_strategy_path(path: &str) -> String {
        format!("tests/content/relative_strength_index/strategy/{}", path)
    }

    fn _test_strategy(
        target: &mut RelativeStrengthIndexStrategy,
        target_indicator: &mut RelativeStrengthIndex,
        expected: &[Option<TradeDirection>],
    ) {
        let mut snapshot = ArraySnapshot::<Option<TradeDirection>>::new();
        for _ in target.ctx.clone() {
            let output_indicator = target_indicator.next(());
            let output = target.next(output_indicator);
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn strategy_length_14_close() {
        let (df, ctx) = Fixture::load_ctx(&format_strategy_path("length_14_close.csv"));
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
            &df.test_trade_dir_target(),
        );
    }
}
