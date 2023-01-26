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
    rows: Vec<Vec<Box<dyn Feature>>>,
}

impl FeatureComposer {
    pub fn new() -> Self {
        return FeatureComposer { rows: Vec::new() };
    }

    pub fn push_row(&mut self, features: Vec<Box<dyn Feature>>) {
        self.rows.push(features);
    }

    fn flatten_row(features: &Vec<Box<dyn Feature>>) -> HashMap<String, Option<f64>> {
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
                let values = map.entry(key).or_default();
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

    pub fn to_df(&self) -> DataFrame {
        let map = self.flatten();
        let mut columns: Vec<Series> = Vec::new();

        let keys: Vec<String> = map.keys().map(|s| s.to_string()).collect();
        let mut first_order_keys = keys.clone();
        first_order_keys.retain(|s| {
            s == "time" || s == "open" || s == "high" || s == "low" || s == "close" || s == "volume"
        });
        let mut keys = keys
            .into_iter()
            .filter(|s| !first_order_keys.contains(s))
            .collect::<Vec<_>>();
        keys.sort();
        keys = [&first_order_keys[..], &keys[..]].concat();

        for key in keys {
            let value = map.get(&key).unwrap();
            let series = Series::new(&key, value);
            columns.push(series);
        }

        let df: PolarsResult<DataFrame> = DataFrame::new(columns);

        return df.unwrap();
    }
}
