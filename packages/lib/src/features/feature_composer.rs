use std::collections::HashMap;

use crate::base::{
    component_context::ComponentContext,
    features::{feature::Feature, types::FeatureKind},
};
use polars::{
    prelude::{DataFrame, NamedFrom, PolarsResult},
    series::Series,
};

pub struct FeatureComposer {
    rows: Vec<Vec<Feature>>,
}

impl FeatureComposer {
    pub fn new() -> Self {
        return FeatureComposer { rows: Vec::new() };
    }

    pub fn push_row(&mut self, features: Vec<Feature>) {
        self.rows.push(features);
    }

    fn flatten_row(features: &Vec<Feature>) -> HashMap<String, Option<f64>> {
        let mut map: HashMap<String, Option<f64>> = HashMap::new();

        for feature in features {
            map.extend(feature.flatten());
        }

        return map;
    }

    pub fn flatten(&self) -> HashMap<String, Vec<Option<f64>>> {
        let mut map: HashMap<String, Vec<Option<f64>>> = HashMap::new();

        for row in &self.rows {
            let row_map = FeatureComposer::flatten_row(row);

            for (key, value) in row_map {
                let values = map.entry(key).or_insert(Vec::new());
                values.push(value);
            }
        }

        let mut prev_size: Option<usize> = None;
        for (key, value) in &map {
            if let Some(size) = prev_size {
                assert_eq!(size, value.len());
            }
            prev_size = Some(value.len());
        }

        return map;
    }
}
