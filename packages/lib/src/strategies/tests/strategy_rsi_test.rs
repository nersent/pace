#[cfg(test)]
mod tests {
    use crate::{
        base::{
            component_context::ComponentContext,
            implicit::{
                recursive::{recursive_rsi::RecursiveRSI, recursive_sma::RecursiveSMA},
                source::{Source, SourceKind},
            },
            strategy::types::StrategyActionKind,
            utils::testing::{
                load_test_artifact_with_target, load_test_strategy_artifact_with_target,
                ComponentTestSnapshot,
            },
        },
        indicators::indicator_rsi::{IndicatorRSI, IndicatorRSIConfig},
        strategies::strategy_rsi::{
            StrategyRSI, StrategyRSIConfig, STRATEGY_RSI_DEFAULT_OVERBOUGHT_THRESHOLD,
            STRATEGY_RSI_DEFAULT_OVERSOLD_THRESHOLD,
        },
    };

    fn _test(
        cctx: &mut ComponentContext,
        target: &mut StrategyRSI,
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
        let (_df, ctx, expected) =
            load_test_strategy_artifact_with_target("strategies/rsi/btc_1d_length_14_close.csv");
        _test(
            &mut ctx.clone(),
            &mut StrategyRSI::new(
                ctx.clone(),
                StrategyRSIConfig {
                    overbought_threshold: STRATEGY_RSI_DEFAULT_OVERBOUGHT_THRESHOLD,
                    oversold_threshold: STRATEGY_RSI_DEFAULT_OVERSOLD_THRESHOLD,
                },
                IndicatorRSI::new(
                    ctx.clone(),
                    IndicatorRSIConfig {
                        length: 14,
                        src: Source::from_kind(ctx.clone(), SourceKind::Close),
                    },
                ),
            ),
            &expected,
        );
    }
}
