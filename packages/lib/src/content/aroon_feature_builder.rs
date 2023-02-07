use std::collections::HashMap;

use crate::base::{
    components::component_context::ComponentContext,
    features::{feature::Feature, feature_regions::FeatureTernaryTrendRegions},
    statistics::scale_value_min_max,
    strategy::trade::{trade_direction_to_f64, TradeDirection},
};

use super::{
    aroon_indicator::{AroonIndicatorResult, AROON_MAX_VALUE, AROON_MIN_VALUE},
    aroon_strategy::AroonStrategyMetadata,
};

pub struct AroonFeature {
    pub raw_up: Option<f64>,
    pub raw_down: Option<f64>,
    pub up: Option<f64>,
    pub down: Option<f64>,
    pub trend: Option<f64>,
}

impl Feature for AroonFeature {
    fn flatten(&self) -> HashMap<String, Option<f64>> {
        let mut map = HashMap::from([
            (String::from("up_raw"), self.raw_up),
            (String::from("down_raw"), self.raw_down),
            (String::from("up"), self.up),
            (String::from("down"), self.down),
            (String::from("trend"), self.trend),
        ]);
        return map;
    }
}

pub struct AroonFeatureBuilder {
    ctx: ComponentContext,
}

impl AroonFeatureBuilder {
    pub fn next(
        &mut self,
        aroon: &AroonIndicatorResult,
        aroon_trade: Option<TradeDirection>,
        aroon_strategy_metadata: &AroonStrategyMetadata,
    ) -> AroonFeature {
        self.ctx.assert();

        let min = AROON_MIN_VALUE;
        let max = AROON_MAX_VALUE;

        return AroonFeature {
            raw_up: aroon.up,
            raw_down: aroon.down,
            trend: Some(trade_direction_to_f64(aroon_trade)),
            up: aroon.up.map(|value| {
                return scale_value_min_max(value, min, max);
            }),
            down: aroon.down.map(|value| {
                return scale_value_min_max(value, min, max);
            }),
        };
    }
}

impl AroonFeatureBuilder {
    pub fn new(ctx: ComponentContext) -> Self {
        return AroonFeatureBuilder { ctx: ctx.clone() };
    }
}
