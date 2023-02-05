use std::collections::HashMap;

use crate::base::statistics::{
    scale_value_centered, scale_value_down, scale_value_min_max, scale_value_up,
};

use super::feature::Feature;

#[derive(Debug)]
pub struct FeatureTernaryTrendRegions {
    pub main: Option<f64>,
    pub oversold: Option<f64>,
    pub consolidation: Option<f64>,
    pub overbought: Option<f64>,
}

impl Feature for FeatureTernaryTrendRegions {
    fn flatten(&self) -> HashMap<String, Option<f64>> {
        let mut map = HashMap::from([
            (String::from("main"), self.main),
            (String::from("oversold"), self.oversold),
            (String::from("consolidation"), self.consolidation),
            (String::from("overbought"), self.overbought),
        ]);

        return map;
    }
}

impl FeatureTernaryTrendRegions {
    pub fn new(
        value: Option<f64>,
        min: f64,
        max: f64,
        threshold_oversold: f64,
        threshold_overbought: f64,
    ) -> Self {
        if value.is_none() {
            return FeatureTernaryTrendRegions {
                main: None,
                overbought: None,
                consolidation: None,
                oversold: None,
            };
        }
        let value = value.unwrap();
        return FeatureTernaryTrendRegions {
            main: Some(scale_value_min_max(value, min, max)),
            overbought: Some(scale_value_up(value, threshold_overbought, max)),
            consolidation: Some(scale_value_centered(
                value,
                (max - min) / 2.0,
                threshold_oversold,
                threshold_overbought,
            )),
            oversold: Some(scale_value_down(value, threshold_oversold, min)),
        };
    }
}
