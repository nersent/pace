use std::path::Path;

use crate::polars::io::read_df;

use super::{
    data_provider::{AnyDataProvider, DataProvider},
    in_memory_data_provider::InMemoryDataProvider,
    timeframe::Timeframe,
};

#[derive(Clone, Debug)]
pub struct Asset {
    pub hash: String,
    pub symbol: String,
    pub timeframe: Timeframe,
}

// impl Asset {
//     pub fn from_path(id: &str, timeframe: Timeframe, path: &Path) -> Self {
//         let df = read_df(&path);
//         let data_provider = InMemoryDataProvider::from_df(&df).to_arc();

//         return Self {
//             timeframe,
//             id: id.to_string(),
//         };
//     }
// }
