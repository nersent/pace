use crate::{
    base::{
        component_context::ComponentContext,
        features::{feature::Feature, types::FeatureKind},
        implicit::{
            recursive::{
                recursive_cross_over::RecursiveCrossOver,
                recursive_cross_under::RecursiveCrossUnder,
                recursive_rsi::{RecursiveRSI, RecursiveRSIResult},
            },
            source::Source,
        },
        strategy::types::StrategyActionKind,
    },
    indicators::indicator_rsi::{
        IndicatorRSI, IndicatorRSIResult, INDICATOR_RSI_MAX_VALUE, INDICATOR_RSI_MIN_VALUE,
    },
    strategies::strategy_rsi::StrategyRSI,
    utils::math::{
        clip_value, scale_value_centered, scale_value_down, scale_value_min_max, scale_value_up,
    },
};

pub struct FeatureRSI {
    ctx: ComponentContext,
    strategy: StrategyRSI,
}

impl FeatureRSI {
    pub fn new(ctx: ComponentContext, strategy: StrategyRSI) -> Self {
        return FeatureRSI {
            ctx: ctx.clone(),
            strategy,
        };
    }

    pub fn next(&mut self) -> Feature {
        self.ctx.assert();
        let (strategy, res) = self.strategy.next();
        let rsi = res.rsi;

        return Feature::as_root(
            "rsi",
            Vec::from([
                Feature::as_raw("value", rsi),
                Feature::from_strategy_action(strategy),
                Feature::to_overbought_oversold_regions(
                    "value",
                    rsi,
                    INDICATOR_RSI_MIN_VALUE,
                    INDICATOR_RSI_MAX_VALUE,
                    self.strategy.config.oversold_threshold,
                    self.strategy.config.overbought_threshold,
                ),
                Feature::as_numeric("up", res.up.map(|v| v / INDICATOR_RSI_MAX_VALUE)),
                Feature::as_numeric("down", res.down.map(|v| v / INDICATOR_RSI_MAX_VALUE)),
            ]),
        );
    }
}
