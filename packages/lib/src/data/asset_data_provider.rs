use super::types::Timeframe;

pub trait AssetDataProvider {
    fn get_asset_name(&self) -> &str;
    fn get_timeframe(&self) -> &Timeframe;
    fn get_start_tick(&self) -> usize;
    fn get_end_tick(&self) -> usize;
    fn get_open(&self, index: usize) -> Option<f64>;
    fn get_high(&self, index: usize) -> Option<f64>;
    fn get_low(&self, index: usize) -> Option<f64>;
    fn get_close(&self, index: usize) -> Option<f64>;
    fn get_volume(&self, index: usize) -> Option<f64>;
    fn get_opens(&self, start_index: usize, end_index: usize) -> &[Option<f64>];
    fn get_highs(&self, start_index: usize, end_index: usize) -> &[Option<f64>];
    fn get_lows(&self, start_index: usize, end_index: usize) -> &[Option<f64>];
    fn get_closes(&self, start_index: usize, end_index: usize) -> &[Option<f64>];
    fn get_volumes(&self, start_index: usize, end_index: usize) -> &[Option<f64>];
}
