use std::collections::HashMap;

use crate::{
    base::{
        component_context::ComponentContext,
        features::{
            feature::Feature, feature_builder::FeatureBuilder,
            feature_regions::FeatureTernaryTrendRegions, types::FeatureKind,
        },
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

pub struct FeatureBuilderRSI {
    ctx: ComponentContext,
    strategy: StrategyRSI,
}

pub struct FeatureRSI {
    pub raw: Option<f64>,
    pub action: StrategyActionKind,
    pub regions: FeatureTernaryTrendRegions,
    pub up: Option<f64>,
    pub down: Option<f64>,
}

impl Feature for FeatureRSI {
    fn flatten(&self) -> HashMap<String, Option<f64>> {
        let mut map = HashMap::from([
            (String::from("raw"), self.raw),
            (String::from("up"), self.up),
            (String::from("down"), self.down),
            (String::from("action"), Some(self.action.to_f64())),
        ]);
        map.extend(self.regions.flatten());
        return map;
    }
}

impl FeatureBuilder<FeatureRSI> for FeatureBuilderRSI {
    fn next(&mut self) -> FeatureRSI {
        self.ctx.assert();
        let (action, res) = self.strategy.next();
        let rsi = res.rsi;

        return FeatureRSI {
            raw: rsi,
            action,
            regions: FeatureTernaryTrendRegions::new(
                rsi,
                INDICATOR_RSI_MIN_VALUE,
                INDICATOR_RSI_MAX_VALUE,
                self.strategy.config.oversold_threshold,
                self.strategy.config.overbought_threshold,
            ),
            up: res.up.map(|v| v / INDICATOR_RSI_MAX_VALUE),
            down: res.down.map(|v| v / INDICATOR_RSI_MAX_VALUE),
        };
    }
}

impl FeatureBuilderRSI {
    pub fn new(ctx: ComponentContext, strategy: StrategyRSI) -> Self {
        return FeatureBuilderRSI {
            ctx: ctx.clone(),
            strategy,
        };
    }
}
