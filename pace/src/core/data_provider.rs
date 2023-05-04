use std::{sync::Arc, time::Duration};

use super::timeframe::Timeframe;

/// OHLCV data provider.
pub trait DataProvider: 'static {
    fn get_first_tick(&self) -> usize;
    fn get_last_tick(&self) -> usize;
    fn get_open(&self, index: usize) -> f64;
    fn get_high(&self, index: usize) -> f64;
    fn get_low(&self, index: usize) -> f64;
    fn get_close(&self, index: usize) -> f64;
    fn get_volume(&self, index: usize) -> f64;
    fn get_time(&self, index: usize) -> Option<Duration>;
    fn get_open_for_range(&self, start_index: usize, end_index: usize) -> &[f64];
    fn get_high_for_range(&self, start_index: usize, end_index: usize) -> &[f64];
    fn get_low_for_range(&self, start_index: usize, end_index: usize) -> &[f64];
    fn get_close_for_range(&self, start_index: usize, end_index: usize) -> &[f64];
    fn get_volume_for_range(&self, start_index: usize, end_index: usize) -> &[f64];
    fn find_tick(&self, seconds: u64) -> Option<usize>;
    fn get_timeframe(&self) -> Timeframe;
    fn to_arc(self) -> AnyDataProvider
    where
        Self: Sized + Send + Sync,
    {
        Arc::new(self)
    }
}

pub type AnyDataProvider = Arc<dyn DataProvider + Send + Sync>;
