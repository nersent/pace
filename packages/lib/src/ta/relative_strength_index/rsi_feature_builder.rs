use std::collections::HashMap;

use crate::{
    components::component_context::ComponentContext,
    features::{
        feature::Feature, feature_builder::FeatureBuilder,
        feature_regions::FeatureTernaryTrendRegions,
    },
    strategy::action::StrategyActionKind,
};

use super::{
    rsi_component::{RSI_MAX_VALUE, RSI_MIN_VALUE},
    rsi_strategy::RelativeStrengthIndexStrategy,
};

pub struct RelativeStrengthIndexFeature {
    pub raw: Option<f64>,
    pub action: StrategyActionKind,
    pub regions: FeatureTernaryTrendRegions,
    pub up: Option<f64>,
    pub down: Option<f64>,
}

impl Feature for RelativeStrengthIndexFeature {
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

pub struct RelativeStrengthIndexFeatureBuilder {
    ctx: ComponentContext,
    strategy: RelativeStrengthIndexStrategy,
}

impl FeatureBuilder<RelativeStrengthIndexFeature> for RelativeStrengthIndexFeatureBuilder {
    fn next(&mut self) -> RelativeStrengthIndexFeature {
        self.ctx.assert();
        let (action, res) = self.strategy.next();
        let rsi = res.rsi;

        return RelativeStrengthIndexFeature {
            raw: rsi,
            action,
            regions: FeatureTernaryTrendRegions::new(
                rsi,
                RSI_MIN_VALUE,
                RSI_MAX_VALUE,
                self.strategy.config.threshold_oversold,
                self.strategy.config.threshold_overbought,
            ),
            up: res.up.map(|v| v / RSI_MAX_VALUE),
            down: res.down.map(|v| v / RSI_MAX_VALUE),
        };
    }
}

impl RelativeStrengthIndexFeatureBuilder {
    pub fn new(ctx: ComponentContext, strategy: RelativeStrengthIndexStrategy) -> Self {
        return RelativeStrengthIndexFeatureBuilder {
            ctx: ctx.clone(),
            strategy,
        };
    }
}
