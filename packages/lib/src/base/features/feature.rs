use std::collections::HashMap;

use crate::utils::hashmap::with_prefix;

#[derive(Debug, PartialEq, Clone)]
pub enum FeatureKind {
    Raw,
    Numeric,
    Binary,
    Root,
}

pub trait Feature {
    fn flatten(&self) -> HashMap<String, Option<f64>>;
    fn as_box(self) -> Box<dyn Feature>
    where
        Self: 'static + Sized,
    {
        return Box::new(self);
    }
}

pub struct FeatureNamespace {
    pub name: String,
    feature: Box<dyn Feature>,
}

impl Feature for FeatureNamespace {
    fn flatten(&self) -> HashMap<String, Option<f64>> {
        return with_prefix(self.feature.flatten(), format!("{}_", self.name).as_str());
    }
}

impl FeatureNamespace {
    pub fn new(name: String, feature: Box<dyn Feature>) -> Self {
        return FeatureNamespace { name, feature };
    }
}

pub struct RawFeature {
    pub name: String,
    pub value: Option<f64>,
}

impl Feature for RawFeature {
    fn flatten(&self) -> HashMap<String, Option<f64>> {
        let mut map = HashMap::new();
        map.insert(self.name.clone(), self.value);
        return map;
    }
}

impl RawFeature {
    pub fn new(name: &str, value: Option<f64>) -> Self {
        return RawFeature {
            name: String::from(name),
            value,
        };
    }
}

pub struct CombinedFeatures {
    pub features: Vec<Box<dyn Feature>>,
}

impl Feature for CombinedFeatures {
    fn flatten(&self) -> HashMap<String, Option<f64>> {
        let mut map = HashMap::new();
        for feature in &self.features {
            map.extend(feature.flatten());
        }
        return map;
    }
}

impl CombinedFeatures {
    pub fn new() -> Self {
        return CombinedFeatures {
            features: Vec::new(),
        };
    }

    pub fn push(&mut self, feature: Box<dyn Feature>) {
        self.features.push(feature);
    }
}
