#[cfg(test)]
mod tests {
    use crate::{
        components::{
            component_context::ComponentContext,
            source::{Source, SourceKind},
            testing::ComponentTestSnapshot,
        },
        strategy::action::StrategyActionKind,
        ta::relative_strength_index::{
            rsi_indicator::{RelativeStrengthIndexIndicator, RelativeStrengthIndexIndicatorConfig},
            rsi_strategy::{
                RelativeStrengthIndexStrategy, RelativeStrengthIndexStrategyConfig,
                RSI_STRATEGY_THRESHOLD_OVERBOUGHT, RSI_STRATEGY_THRESHOLD_OVERSOLD,
            },
        },
        testing::fixture::Fixture,
    };

    fn _test(
        cctx: &mut ComponentContext,
        target: &mut RelativeStrengthIndexStrategy,
        expected: &[Option<StrategyActionKind>],
    ) {
        let mut snapshot = ComponentTestSnapshot::<StrategyActionKind>::new();
        for cctx in cctx {
            let (result, _) = target.next();
            snapshot.push(Some(result));
        }
        snapshot.assert(expected);
    }

    #[test]
    fn test_rsi_btc_1d_length_14_close() {
        let (_df, ctx, expected) = Fixture::strategy(
            "ta/relative_strength_index/tests/fixtures/rsi_strategy/btc_1d_length_14_close.csv",
        );
        _test(
            &mut ctx.clone(),
            &mut RelativeStrengthIndexStrategy::new(
                ctx.clone(),
                RelativeStrengthIndexStrategyConfig {
                    threshold_oversold: RSI_STRATEGY_THRESHOLD_OVERSOLD,
                    threshold_overbought: RSI_STRATEGY_THRESHOLD_OVERBOUGHT,
                },
                RelativeStrengthIndexIndicator::new(
                    ctx.clone(),
                    RelativeStrengthIndexIndicatorConfig {
                        length: 14,
                        src: Source::from_kind(ctx.clone(), SourceKind::Close),
                    },
                ),
            ),
            &expected,
        );
    }
}
