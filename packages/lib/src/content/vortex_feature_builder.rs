use std::collections::HashMap;

use crate::base::{
    components::component_context::ComponentContext,
    features::{feature::Feature, feature_regions::FeatureTernaryTrendRegions},
    statistics::{clip_value, scale_value_min_max},
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
    relative_vigor_index_indicator::{
        RelativeVigorIndexIndicatorResult, RELATIVE_VIGOR_INDEX_MAX_VALUE,
        RELATIVE_VIGOR_INDEX_MIN_VALUE,
    },
    vortex_indicator::VortexIndicatorResult,
};

pub struct VortexFeature {
    pub raw_plus: Option<f64>,
    pub raw_minus: Option<f64>,
    pub plus: Option<f64>,
    pub minus: Option<f64>,
    pub trend: Option<f64>,
}

impl Feature for VortexFeature {
    fn flatten(&self) -> HashMap<String, Option<f64>> {
        let mut map = HashMap::from([
            (String::from("plus_raw"), self.raw_plus),
            (String::from("minus_raw"), self.raw_minus),
            (String::from("plus"), self.plus),
            (String::from("minus"), self.minus),
            (String::from("trend"), self.trend),
        ]);
        return map;
    }
}

pub struct VortexFeatureBuilder {
    ctx: ComponentContext,
}

impl VortexFeatureBuilder {
    pub fn next(
        &mut self,
        vi: &VortexIndicatorResult,
        vi_trade: Option<TradeDirection>,
    ) -> VortexFeature {
        self.ctx.assert();

        let min = 0.0;
        let max = 1.5;

        return VortexFeature {
            raw_plus: vi.plus,
            raw_minus: vi.minus,
            plus: vi
                .plus
                .map(|v| clip_value(scale_value_min_max(v, min, max), -1.0, 1.0)),
            minus: vi
                .minus
                .map(|v| clip_value(scale_value_min_max(v, min, max), -1.0, 1.0)),
            trend: Some(trade_direction_to_f64(vi_trade)),
        };
    }
}

impl VortexFeatureBuilder {
    pub fn new(ctx: ComponentContext) -> Self {
        return VortexFeatureBuilder { ctx: ctx.clone() };
    }
}
