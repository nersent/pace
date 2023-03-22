use std::{path::Path, sync::Arc};

use polars::prelude::DataFrame;

use crate::{
    core::{
        context::Context, data_provider::DataProvider,
        in_memory_data_provider::InMemoryDataProvider,
    },
    polars::{io::read_df, series::SeriesCastUtils},
    strategy::trade::TradeDirection,
};

pub struct Fixture {}

impl Fixture {
    pub fn load_ctx(path: &str) -> (DataFrame, Context) {
        let mut normalized_path = Path::new("fixtures").join(path);
        let test_mode = std::env::var("NEXTEST").is_ok();

        if test_mode {
            normalized_path = Path::new("../").join(normalized_path);
        }

        let df = read_df(&normalized_path);
        let ctx = Context::new(InMemoryDataProvider::from_df(&df).to_arc());
        return (df, ctx);
    }
}

pub trait DataFrameFixtureUtils {
    fn test_target(&self) -> Vec<Option<f64>>;
    fn test_trade_dir_target(&self) -> Vec<Option<TradeDirection>>;
}

impl DataFrameFixtureUtils for DataFrame {
    fn test_target(&self) -> Vec<Option<f64>> {
        return self.column("_target_").unwrap().to_f64();
    }

    fn test_trade_dir_target(&self) -> Vec<Option<TradeDirection>> {
        return self.column("_target_").unwrap().to_trade_dir();
    }
}
