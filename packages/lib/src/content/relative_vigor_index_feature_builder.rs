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
};

pub struct RelativeVigorIndexFeature {
    pub raw_rvi: Option<f64>,
    pub raw_rvi_s: Option<f64>,
    pub rvi: Option<f64>,
    pub sig: Option<f64>,
    pub trend: Option<f64>,
}

impl Feature for RelativeVigorIndexFeature {
    fn flatten(&self) -> HashMap<String, Option<f64>> {
        let mut map = HashMap::from([
            (String::from("rvi_raw"), self.raw_rvi),
            (String::from("rvi_s_raw"), self.raw_rvi_s),
            (String::from("rvi"), self.rvi),
            (String::from("sig"), self.sig),
            (String::from("trend"), self.trend),
        ]);
        return map;
    }
}

pub struct RelativeVigorIndexFeatureBuilder {
    ctx: ComponentContext,
}

impl RelativeVigorIndexFeatureBuilder {
    pub fn next(
        &mut self,
        rvgi: &RelativeVigorIndexIndicatorResult,
        rvgi_trade: Option<TradeDirection>,
    ) -> RelativeVigorIndexFeature {
        self.ctx.assert();

        let min = RELATIVE_VIGOR_INDEX_MIN_VALUE;
        let max = RELATIVE_VIGOR_INDEX_MAX_VALUE;

        return RelativeVigorIndexFeature {
            raw_rvi: rvgi.rvi,
            raw_rvi_s: rvgi.sig,
            rvi: rvgi.rvi.map(|v| clip_value(v, -1.0, 1.0)),
            sig: rvgi.sig.map(|v| clip_value(v, -1.0, 1.0)),
            trend: Some(trade_direction_to_f64(rvgi_trade)),
        };
    }
}

impl RelativeVigorIndexFeatureBuilder {
    pub fn new(ctx: ComponentContext) -> Self {
        return RelativeVigorIndexFeatureBuilder { ctx: ctx.clone() };
    }
}
