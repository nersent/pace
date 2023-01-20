use std::collections::HashMap;

use crate::{
    base::strategy::types::StrategyActionKind, features::utils::compute_regions,
    utils::math::clip_value,
};

use super::types::FeatureKind;

pub struct Feature {
    pub name: String,
    pub kind: FeatureKind,
    pub value: Option<f64>,
    pub children: Option<Vec<Feature>>,
}

impl Feature {
    pub fn as_root(name: &str, children: Vec<Feature>) -> Self {
        return Feature {
            name: String::from(name),
            kind: FeatureKind::Root,
            value: None,
            children: Some(children),
        };
    }

    pub fn as_binary(name: &str, value: Option<f64>) -> Self {
        return Feature {
            name: String::from(name),
            kind: FeatureKind::Binary,
            value,
            children: None,
        };
    }

    pub fn as_numeric(name: &str, value: Option<f64>) -> Self {
        return Feature {
            name: String::from(name),
            kind: FeatureKind::Numeric,
            value: match value {
                Some(v) => Some(clip_value(v, -1.0, 1.0)),
                None => None,
            },
            children: None,
        };
    }

    pub fn as_raw(name: &str, value: Option<f64>) -> Self {
        return Feature {
            name: String::from(name),
            kind: FeatureKind::Raw,
            value,
            children: None,
        };
    }

    pub fn from_strategy_action(action: Option<StrategyActionKind>) -> Self {
        return Feature::as_binary(
            "action",
            match action {
                Some(StrategyActionKind::Long) => Some(1.0),
                Some(StrategyActionKind::Short) => Some(-1.0),
                None => Some(0.0),
                _ => panic!("Unexpected strategy action"),
            },
        );
    }

    pub fn to_overbought_oversold_regions(
        name: &str,
        value: Option<f64>,
        min: f64,
        max: f64,
        threshold_oversold: f64,
        threshold_overbought: f64,
    ) -> Self {
        let regions = compute_regions(value, min, max, threshold_oversold, threshold_overbought);
        return Feature::as_root(
            name,
            Vec::from([
                Feature::as_numeric("main", regions.main),
                Feature::as_numeric("overbought", regions.overbought),
                Feature::as_numeric("consolidation", regions.consolidation),
                Feature::as_numeric("oversold", regions.oversold),
            ]),
        );
    }

    pub fn flatten_feature(feature: &Feature, path: &[&str]) -> HashMap<String, Option<f64>> {
        let mut current_path = path.to_vec();
        current_path.push(&feature.name);

        let mut map: HashMap<String, Option<f64>> = HashMap::new();

        if feature.kind == FeatureKind::Root {
            for sub_feature in feature.children.as_ref().unwrap() {
                let sub_map = Self::flatten_feature(sub_feature, current_path.as_slice());
                map.extend(sub_map);
            }
            return map;
        }

        let suffix = match feature.kind {
            FeatureKind::Raw => "raw",
            FeatureKind::Numeric => "num",
            FeatureKind::Binary => "bin",
            _ => panic!("Unexpected feature kind"),
        };

        let full_name = format!("{}_{}", current_path.join("_"), suffix);
        map.insert(full_name, feature.value);
        return map;
    }

    pub fn flatten(&self) -> HashMap<String, Option<f64>> {
        return Self::flatten_feature(self, &[]);
    }
}
