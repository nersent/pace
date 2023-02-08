use std::{path::Path, sync::Arc};

use crate::{
    base::{
        asset::{
            asset_data_provider::AssetDataProvider,
            in_memory_asset_data_provider::InMemoryAssetDataProvider, timeframe::Timeframe,
        },
        components::component_context::ComponentContext,
    },
    utils::polars::read_df,
};
use pyo3::prelude::*;

#[pyclass(name = "AssetDataProvider")]
pub struct PyAssetDataProvider {
    pub asset: Arc<dyn AssetDataProvider + 'static + Send + Sync>,
}

#[pymethods]
impl PyAssetDataProvider {
    #[new]
    pub fn new(path: String, asset_name: String, timeframe: usize) -> Self {
        let timeframe = Timeframe::try_from(timeframe).unwrap();

        let path = Path::new(&path);

        let df = read_df(&path);
        let asset = Arc::from(InMemoryAssetDataProvider::from_df(
            &df,
            &asset_name,
            timeframe,
        ));

        return Self { asset };
    }

    pub fn get_asset_name(&self) -> String {
        return self.asset.get_asset_name().to_string();
    }

    pub fn get_timeframe(&self) -> usize {
        return Timeframe::try_into(*self.asset.get_timeframe()).unwrap();
    }

    pub fn get_start_tick(&self) -> usize {
        return self.asset.get_start_tick();
    }

    pub fn get_end_tick(&self) -> usize {
        return self.asset.get_end_tick();
    }
}
