use std::time::Duration;

use polars::prelude::{DataFrame, TimeUnit};

use crate::data::polars::SeriesCastUtils;

use super::{asset_data_provider::AssetDataProvider, timeframe::Timeframe};

pub struct InMemoryAssetDataProvider {
    pub asset_name: String,
    pub timeframe: Timeframe,
    open: Vec<Option<f64>>,
    high: Vec<Option<f64>>,
    low: Vec<Option<f64>>,
    close: Vec<Option<f64>>,
    volume: Vec<Option<f64>>,
    time: Vec<Option<Duration>>,
    start_tick: usize,
    end_tick: usize,
}

impl AssetDataProvider for InMemoryAssetDataProvider {
    fn get_asset_name(&self) -> &str {
        return &self.asset_name;
    }

    fn get_timeframe(&self) -> &Timeframe {
        return &self.timeframe;
    }

    fn get_start_tick(&self) -> usize {
        return self.start_tick;
    }

    fn get_end_tick(&self) -> usize {
        return self.end_tick;
    }

    fn get_open(&self, index: usize) -> Option<f64> {
        return self.open[index];
    }

    fn get_high(&self, index: usize) -> Option<f64> {
        return self.high[index];
    }

    fn get_low(&self, index: usize) -> Option<f64> {
        return self.low[index];
    }

    fn get_close(&self, index: usize) -> Option<f64> {
        return self.close[index];
    }

    fn get_volume(&self, index: usize) -> Option<f64> {
        return self.volume[index];
    }

    fn get_time(&self, index: usize) -> Option<Duration> {
        return self.time[index];
    }

    fn get_opens(&self, start_index: usize, end_index: usize) -> &[Option<f64>] {
        return &self.open[start_index..end_index + 1];
    }

    fn get_highs(&self, start_index: usize, end_index: usize) -> &[Option<f64>] {
        return &self.high[start_index..end_index + 1];
    }

    fn get_lows(&self, start_index: usize, end_index: usize) -> &[Option<f64>] {
        return &self.low[start_index..end_index + 1];
    }

    fn get_closes(&self, start_index: usize, end_index: usize) -> &[Option<f64>] {
        return &self.close[start_index..end_index + 1];
    }

    fn get_volumes(&self, start_index: usize, end_index: usize) -> &[Option<f64>] {
        return &self.volume[start_index..end_index + 1];
    }
}

impl InMemoryAssetDataProvider {
    pub fn new(
        asset_name: &str,
        timeframe: Timeframe,
        open: Vec<Option<f64>>,
        high: Vec<Option<f64>>,
        low: Vec<Option<f64>>,
        close: Vec<Option<f64>>,
        volume: Vec<Option<f64>>,
        time: Vec<Option<Duration>>,
    ) -> Self {
        let start_tick = 0;
        let end_tick = close.len() - 1;

        return InMemoryAssetDataProvider {
            asset_name: asset_name.to_string(),
            timeframe,
            open,
            high,
            low,
            close,
            volume,
            start_tick,
            end_tick,
            time,
        };
    }

    pub fn from_values(values: Vec<Option<f64>>) -> Self {
        return InMemoryAssetDataProvider {
            asset_name: "IN_MEMORY_DATA_PROVIDER_EXACT".to_string(),
            timeframe: Timeframe::OneDay,
            open: values.clone(),
            high: values.clone(),
            low: values.clone(),
            close: values.clone(),
            volume: values.clone(),
            start_tick: 0,
            end_tick: values.len() - 1,
            time: vec![None; values.len()],
        };
    }

    pub fn from_df(df: &DataFrame, asset_name: &str, timeframe: Timeframe) -> Self {
        let open = df.column("open").unwrap().to_f64();
        let high = df.column("high").unwrap().to_f64();
        let low = df.column("low").unwrap().to_f64();
        let close = df.column("close").unwrap().to_f64();
        let volume = df.column("volume").unwrap().to_f64();
        let time = df.column("time").unwrap().to_duration();

        return InMemoryAssetDataProvider::new(
            asset_name, timeframe, open, high, low, close, volume, time,
        );
    }
}
