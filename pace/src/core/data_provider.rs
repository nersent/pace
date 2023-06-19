use std::{sync::Arc, time::Duration};

use super::timeframe::Timeframe;

#[derive(Debug, Clone, Copy)]
pub struct SymInfo {
    /// The tick size is the smallest possible price change an instrument can have [1]. In other words, when the price of an instrument fluctuates, it always changes with the size of at least one tick.
    // Stocks usually have a tick size of one cent (0.01). Most spot forex symbols trade in 0.00001 increments. The E-mini S&P 500 future uses a tick size of 0.25, while the EuroStoxx 50 future works with a value of 0.5.
    /// https://www.tradingcode.net/tradingview/instrument-minimum-tick/
    pub min_tick: f64,
    // https://www.tradingcode.net/tradingview/equity-percent-default-order/#order-size-formula
    pub min_qty: f64,
}

impl Default for SymInfo {
    fn default() -> Self {
        Self {
            min_tick: f64::NAN,
            min_qty: f64::NAN,
        }
    }
}

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
    fn get_sym_info(&self) -> Option<&SymInfo> {
        return None;
    }
}

pub type AnyDataProvider = Arc<dyn DataProvider + Send + Sync>;
