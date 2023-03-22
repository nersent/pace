use std::sync::Arc;

use polars::prelude::DataFrame;

use crate::core::{data_provider::DataProvider, in_memory_data_provider::InMemoryDataProvider};

use super::series::SeriesCastUtils;

impl InMemoryDataProvider {
    pub fn from_df(df: &DataFrame) -> Self {
        let open = df.column("open").unwrap().to_f64();
        let high = df.column("high").unwrap().to_f64();
        let low = df.column("low").unwrap().to_f64();
        let close = df.column("close").unwrap().to_f64();
        let volume = df.column("volume").unwrap().to_f64();
        let time = df.column("time").unwrap().to_duration();

        return Self::new(open, high, low, close, volume, time);
    }
}
