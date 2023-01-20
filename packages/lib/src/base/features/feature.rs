use std::collections::HashMap;

use crate::{
    base::strategy::types::StrategyActionKind,
    utils::{hashmap::with_prefix, math::clip_value},
};

use super::types::FeatureKind;

pub trait Feature {
    fn flatten(&self) -> HashMap<String, Option<f64>>;
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
