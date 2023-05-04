use std::{collections::HashMap, path::Path};

use polars::{
    prelude::{DataFrame, NamedFrom},
    series::Series,
};

use crate::{core::trend::Trend, polars::io::save_df, strategy::trade::StrategySignal};

#[derive(Debug, Clone)]
pub enum FeatureValue {
    Continous(f64),
    Discrete(bool),
    Raw(String),
    Trend(Trend),
    Signal(StrategySignal),
}

impl Into<String> for FeatureValue {
    fn into(self) -> String {
        match self {
            FeatureValue::Continous(value) => value.to_string(),
            FeatureValue::Discrete(value) => (value as i32).to_string(),
            FeatureValue::Raw(value) => value,
            FeatureValue::Trend(value) => Into::<f64>::into(value).to_string(),
            FeatureValue::Signal(value) => Into::<f64>::into(value).to_string(),
            _ => panic!("Cannot convert feature to f64"),
        }
    }
}

pub trait FeatureBuilder {
    fn flatten(&self) -> HashMap<String, FeatureValue>;
}

pub struct FeatureRegistry {
    pub map: HashMap<String, Vec<FeatureValue>>,
}

impl FeatureRegistry {
    pub fn new() -> Self {
        return Self {
            map: HashMap::new(),
        };
    }

    pub fn push(&mut self, id: &str, value: FeatureValue) {
        let values = self.map.entry(id.to_string()).or_insert(Vec::new());
        values.push(value);
    }

    pub fn get(&self, id: &str) -> Option<&Vec<FeatureValue>> {
        return self.map.get(id);
    }

    pub fn build_series(&self) -> Vec<Series> {
        let mut list: Vec<Series> = Vec::new();

        for (id, values) in &self.map {
            let values_as_str: Vec<String> = values
                .iter()
                .map(|v| Into::<String>::into(v.clone()))
                .collect();
            let series = Series::new(id, values_as_str);
            list.push(series);
        }

        return list;
    }

    pub fn build_df(&self) -> DataFrame {
        let series_map = self.build_series();
        let df = DataFrame::new(series_map).unwrap();
        return df;
    }

    pub fn save(&self, path: &Path) {
        let mut df = self.build_df();
        save_df(&mut df, path);
    }
}
