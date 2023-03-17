use std::time::Duration;

/// OHLCV data provider.
pub trait DataProvider {
    fn get_start_tick(&self) -> usize;
    fn get_end_tick(&self) -> usize;
    fn get_open(&self, index: usize) -> Option<f64>;
    fn get_high(&self, index: usize) -> Option<f64>;
    fn get_low(&self, index: usize) -> Option<f64>;
    fn get_close(&self, index: usize) -> Option<f64>;
    fn get_volume(&self, index: usize) -> Option<f64>;
    fn get_time(&self, index: usize) -> Option<Duration>;
    fn get_open_for_range(&self, start_index: usize, end_index: usize) -> &[Option<f64>];
    fn get_high_for_range(&self, start_index: usize, end_index: usize) -> &[Option<f64>];
    fn get_low_for_range(&self, start_index: usize, end_index: usize) -> &[Option<f64>];
    fn get_close_for_range(&self, start_index: usize, end_index: usize) -> &[Option<f64>];
    fn get_volume_for_range(&self, start_index: usize, end_index: usize) -> &[Option<f64>];
}
