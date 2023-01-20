use crate::utils::math::{
    scale_value_centered, scale_value_down, scale_value_min_max, scale_value_up,
};

#[derive(Debug)]
pub struct FeatureRegions {
    pub main: Option<f64>,
    pub oversold: Option<f64>,
    pub consolidation: Option<f64>,
    pub overbought: Option<f64>,
}

pub fn compute_regions(
    value: Option<f64>,
    min: f64,
    max: f64,
    threshold_oversold: f64,
    threshold_overbought: f64,
) -> FeatureRegions {
    if value.is_none() {
        return FeatureRegions {
            main: None,
            overbought: None,
            consolidation: None,
            oversold: None,
        };
    }
    let value = value.unwrap();
    return FeatureRegions {
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
