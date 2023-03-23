use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

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
    pub fn load_ctx(path: &Path) -> (DataFrame, Context) {
        let df = read_df(&path);
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
