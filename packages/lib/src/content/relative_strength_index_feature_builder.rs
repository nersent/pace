use std::collections::HashMap;

use crate::base::{
    components::component_context::ComponentContext,
    features::{feature::Feature, feature_regions::FeatureTernaryTrendRegions},
    statistics::scale_value_min_max,
    strategy::action::StrategyActionKind,
    ta::rsi_component::{RelativeStrengthIndexComponentMetadata, RSI_MAX_VALUE, RSI_MIN_VALUE},
};

use super::relative_strength_index_strategy::{
    RelativeStrengthIndexStrategy, RelativeStrengthIndexStrategyConfig,
};

pub struct RelativeStrengthIndexFeature {
    pub raw: Option<f64>,
    pub main: Option<f64>,
}

impl Feature for RelativeStrengthIndexFeature {
    fn flatten(&self) -> HashMap<String, Option<f64>> {
        let mut map = HashMap::from([
            (String::from("raw"), self.raw),
            (String::from("main"), self.main),
        ]);
        return map;
    }
}

pub struct RelativeStrengthIndexFeatureBuilder {
    ctx: ComponentContext,
}

impl RelativeStrengthIndexFeatureBuilder {
    pub fn next(
        &mut self,
        rsi: Option<f64>,
        rsi_metadata: &RelativeStrengthIndexComponentMetadata,
        rsi_strategy_action: StrategyActionKind,
        rsi_strategy_config: &RelativeStrengthIndexStrategyConfig,
    ) -> RelativeStrengthIndexFeature {
        self.ctx.assert();

        let min = RSI_MIN_VALUE;
        let max = RSI_MAX_VALUE;

        return RelativeStrengthIndexFeature {
            raw: rsi,
            main: rsi.map(|value| {
                return scale_value_min_max(value, min, max);
            }),
        };
    }
}

impl RelativeStrengthIndexFeatureBuilder {
    pub fn new(ctx: ComponentContext) -> Self {
        return RelativeStrengthIndexFeatureBuilder { ctx: ctx.clone() };
    }
}
