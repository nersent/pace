use std::{collections::HashMap, rc::Rc, sync::Arc};

use super::asset_data_provider::AssetDataProvider;

pub struct AssetDataProviderManager {
    pub providers: HashMap<String, Arc<dyn AssetDataProvider + 'static + Send + Sync>>,
}

impl AssetDataProviderManager {
    pub fn new() -> AssetDataProviderManager {
        return AssetDataProviderManager {
            providers: HashMap::new(),
        };
    }

    pub fn add(&mut self, id: &str, provider: Arc<dyn AssetDataProvider + 'static + Send + Sync>) {
        assert!(
            !self.providers.contains_key(id),
            "AssetDataProviderManager: provider with id {} already exists",
            id
        );
        self.providers.insert(id.to_string(), provider);
    }

    pub fn get(&self, id: &str) -> Arc<dyn AssetDataProvider + 'static + Send + Sync> {
        assert!(
            self.providers.contains_key(id),
            "AssetDataProviderManager: provider with id {} does not exist",
            id
        );
        return Arc::clone(self.providers.get(id).unwrap());
    }
}
