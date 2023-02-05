#[cfg(test)]
mod tests {
    use crate::{
        base::{
            asset::source::{Source, SourceKind},
            components::{
                component_context::ComponentContext,
                testing::{ComponentTestSnapshot, Fixture},
            },
            strategy::action::StrategyActionKind,
        },
        content::{
            relative_strength_index_indicator::{
                RelativeStrengthIndexIndicator, RelativeStrengthIndexIndicatorConfig,
            },
            relative_strength_index_strategy::{
                RelativeStrengthIndexStrategy, RelativeStrengthIndexStrategyConfig,
                RSI_STRATEGY_THRESHOLD_OVERBOUGHT, RSI_STRATEGY_THRESHOLD_OVERSOLD,
            },
        },
    };

    fn format_path(path: &str) -> String {
        format!(
            "content/tests/fixtures/relative_strength_index/strategy/{}",
            path
        )
    }

    fn _test(
        cctx: &mut ComponentContext,
        target: &mut RelativeStrengthIndexStrategy,
        target_indicator: &mut RelativeStrengthIndexIndicator,
        expected: &[Option<StrategyActionKind>],
    ) {
        let mut snapshot = ComponentTestSnapshot::<StrategyActionKind>::new();
        for cctx in cctx {
            let output_indicator = target_indicator.next();
            let output = target.next(output_indicator);
            snapshot.push(Some(output));
        }
        snapshot.assert(expected);
    }

    #[test]
    fn length_14_close() {
        let (_df, ctx, expected) = Fixture::strategy(&format_path("length_14_close.csv"));
        _test(
            &mut ctx.clone(),
            &mut RelativeStrengthIndexStrategy::new(
                ctx.clone(),
                RelativeStrengthIndexStrategyConfig {
                    threshold_oversold: RSI_STRATEGY_THRESHOLD_OVERSOLD,
                    threshold_overbought: RSI_STRATEGY_THRESHOLD_OVERBOUGHT,
                },
            ),
            &mut RelativeStrengthIndexIndicator::new(
                ctx.clone(),
                RelativeStrengthIndexIndicatorConfig {
                    length: 14,
                    src: Source::from_kind(ctx.clone(), SourceKind::Close),
                },
            ),
            &expected,
        );
    }
}
