#[cfg(test)]
mod tests {
    use crate::{
        components::{component::Component, src::SrcKind, src_component::SrcComponent},
        content::{
            relative_strength_index_indicator::{RsiIndicator, RsiIndicatorConfig},
            relative_strength_index_strategy::{
                RsiStrategy, RsiStrategyConfig, RSI_THRESHOLD_OVERBOUGHT, RSI_THRESHOLD_OVERSOLD,
            },
        },
        strategy::trade::TradeDirection,
        testing::{
            array_snapshot::ArraySnapshot,
            fixture::{DataFrameFixtureUtils, Fixture},
        },
    };

    fn format_path(path: &str) -> String {
        format!("tests/content/relative_strength_index/strategy/{}", path)
    }

    fn _test(
        target: &mut RsiStrategy,
        target_indicator: &mut RsiIndicator,
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
    fn length_14_close() {
        let (df, ctx) = Fixture::load_ctx(&format_path("length_14_close.csv"));
        _test(
            &mut RsiStrategy::new(
                ctx.clone(),
                RsiStrategyConfig {
                    threshold_oversold: RSI_THRESHOLD_OVERSOLD,
                    threshold_overbought: RSI_THRESHOLD_OVERBOUGHT,
                },
            ),
            &mut RsiIndicator::new(
                ctx.clone(),
                RsiIndicatorConfig {
                    length: 14,
                    src: SrcComponent::build(ctx.clone(), SrcKind::Close),
                },
            ),
            &df.test_trade_dir_target(),
        );
    }
}
