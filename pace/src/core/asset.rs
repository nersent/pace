use std::{
    collections::{hash_map::Values, HashMap},
    fmt::{Display, Formatter},
    path::Path,
    sync::Arc,
};

use crate::polars::io::read_df;

use super::{
    data_provider::{AnyDataProvider, DataProvider},
    in_memory_data_provider::InMemoryDataProvider,
    timeframe::Timeframe,
};

#[derive(Debug, Clone)]
pub struct Asset {
    pub id: String,
    pub symbol: String,
    pub timeframe: Timeframe,
}

pub struct AssetRegistryItem {
    pub asset: Asset,
    pub data_provider: Option<AnyDataProvider>,
}

impl Display for AssetRegistryItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let tf: String = self.asset.timeframe.into();
        return write!(f, "{}.{} ({})", self.asset.id, self.asset.symbol, tf);
    }
}

pub struct AssetRegistry {
    pub map: HashMap<String, AssetRegistryItem>,
}

impl AssetRegistry {
    pub fn new() -> Self {
        return Self {
            map: HashMap::new(),
        };
    }

    pub fn add(&mut self, asset: AssetRegistryItem) {
        self.map.insert(asset.asset.id.clone(), asset);
    }

    pub fn remove(&mut self, asset: &AssetRegistryItem) {
        self.map.remove(&asset.asset.id.clone());
    }

    pub fn get_data_provider(&self, asset_id: &str) -> Option<AnyDataProvider> {
        let asset = self.map.get(asset_id).unwrap();
        return asset.data_provider.as_ref().map(|x| Arc::clone(x));
    }

    pub fn print(&self) {
        println!("Assets:");
        for asset in self.map.values() {
            println!("  - {}", asset.asset.id);
        }
        println!();
    }
}
