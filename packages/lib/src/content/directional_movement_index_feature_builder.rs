use std::collections::HashMap;

use crate::base::{
    components::component_context::ComponentContext,
    features::{feature::Feature, feature_regions::FeatureTernaryTrendRegions},
    statistics::scale_value_min_max,
    strategy::trade::{trade_direction_to_f64, TradeDirection},
};

use super::{
    connors_relative_strength_index_indicator::{
        CONNORS_RELATIVE_STRENGTH_INDEX_MAX_VALUE, CONNORS_RELATIVE_STRENGTH_INDEX_MIN_VALUE,
    },
    connors_relative_strength_index_strategy::ConnorsRelativeStrengthIndexStrategyConfig,
    directional_movement_index_indicator::{
        DirectionalMovementIndexIndicatorResult, DIRECTIONAL_MOVEMENT_INDEX_MAX_VALUE,
        DIRECTIONAL_MOVEMENT_INDEX_MIN_VALUE,
    },
    directional_movement_index_strategy::DirectionalMovementIndexStrategyConfig,
    relative_strength_index_strategy::{
        RelativeStrengthIndexStrategy, RelativeStrengthIndexStrategyConfig,
    },
};

pub struct DirectionalMovementIndexFeature {
    pub raw_adx: Option<f64>,
    pub raw_plus: Option<f64>,
    pub raw_minus: Option<f64>,
    pub adx: Option<f64>,
    pub plus: Option<f64>,
    pub minus: Option<f64>,
    pub trend: Option<f64>,
}

impl Feature for DirectionalMovementIndexFeature {
    fn flatten(&self) -> HashMap<String, Option<f64>> {
        let mut map = HashMap::from([
            (String::from("adx_raw"), self.raw_adx),
            (String::from("plus_raw"), self.raw_plus),
            (String::from("minus_raw"), self.raw_minus),
            (String::from("adx"), self.adx),
            (String::from("plus"), self.plus),
            (String::from("minus"), self.minus),
            (String::from("trend"), self.trend),
        ]);
        return map;
    }
}

pub struct DirectionalMovementIndexFeatureBuilder {
    ctx: ComponentContext,
}

impl DirectionalMovementIndexFeatureBuilder {
    pub fn next(
        &mut self,
        dmi: &DirectionalMovementIndexIndicatorResult,
        dmi_trade: Option<TradeDirection>,
        dmi_strategy_config: &DirectionalMovementIndexStrategyConfig,
    ) -> DirectionalMovementIndexFeature {
        self.ctx.assert();

        let min = DIRECTIONAL_MOVEMENT_INDEX_MIN_VALUE;
        let max = DIRECTIONAL_MOVEMENT_INDEX_MAX_VALUE;

        return DirectionalMovementIndexFeature {
            raw_adx: dmi.adx,
            raw_plus: dmi.plus,
            raw_minus: dmi.minus,
            adx: dmi.adx.map(|value| scale_value_min_max(value, min, max)),
            plus: dmi.plus.map(|value| scale_value_min_max(value, min, max)),
            minus: dmi.minus.map(|value| scale_value_min_max(value, min, max)),
            trend: Some(trade_direction_to_f64(dmi_trade)),
        };
    }
}

impl DirectionalMovementIndexFeatureBuilder {
    pub fn new(ctx: ComponentContext) -> Self {
        return DirectionalMovementIndexFeatureBuilder { ctx: ctx.clone() };
    }
}
