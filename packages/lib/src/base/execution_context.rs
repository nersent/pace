use chrono::{DateTime, NaiveDateTime};
use std::{rc::Rc, sync::Arc, time::Duration};

use super::{
    asset::asset_data_provider::AssetDataProvider,
    ta::ma::{compute_hl2, compute_hlc3},
};

pub struct ExecutionContext {
    pub asset_data_provider: Arc<dyn AssetDataProvider + 'static + Send + Sync>,
    pub current_tick: usize,
    start_tick: usize,
    end_tick: usize,
    is_running: bool,
}

impl ExecutionContext {
    pub fn new(
        asset_data_provider: Arc<dyn AssetDataProvider + 'static + Send + Sync>,
        start_tick: usize,
        end_tick: usize,
    ) -> Self {
        assert!(
            start_tick < end_tick,
            "ExecutionContext: start_tick must be less than end_tick"
        );

        return ExecutionContext {
            current_tick: start_tick,
            start_tick,
            end_tick,
            asset_data_provider,
            is_running: false,
        };
    }

    pub fn from_asset(
        asset_data_provider: Arc<dyn AssetDataProvider + 'static + Send + Sync>,
    ) -> Self {
        let start_tick = asset_data_provider.get_start_tick();
        let end_tick = asset_data_provider.get_end_tick();
        return ExecutionContext {
            current_tick: start_tick,
            start_tick,
            end_tick,
            asset_data_provider,
            is_running: false,
        };
    }

    pub fn start_tick(&self) -> usize {
        return self.start_tick;
    }

    pub fn end_tick(&self) -> usize {
        return self.end_tick;
    }

    pub fn count_ticks(&self) -> usize {
        return self.end_tick - self.start_tick + 1;
    }

    pub fn next(&mut self) -> bool {
        if !self.is_running {
            self.is_running = true;
            return true;
        }
        self.current_tick += 1;
        return self.current_tick <= self.end_tick;
    }

    pub fn open(&self) -> Option<f64> {
        return self.asset_data_provider.get_open(self.current_tick);
    }

    pub fn high(&self) -> Option<f64> {
        return self.asset_data_provider.get_high(self.current_tick);
    }

    pub fn low(&self) -> Option<f64> {
        return self.asset_data_provider.get_low(self.current_tick);
    }

    pub fn close(&self) -> Option<f64> {
        return self.asset_data_provider.get_close(self.current_tick);
    }

    pub fn volume(&self) -> Option<f64> {
        return self.asset_data_provider.get_volume(self.current_tick);
    }

    pub fn time(&self) -> Option<Duration> {
        return self.asset_data_provider.get_time(self.current_tick);
    }

    pub fn datetime(&self) -> Option<NaiveDateTime> {
        return self
            .time()
            .map(|time| NaiveDateTime::from_timestamp_millis(time.as_millis() as i64).unwrap());
    }

    pub fn hl2(&self) -> Option<f64> {
        let high = self.high();
        let low = self.low();
        match (high, low) {
            (Some(high), Some(low)) => return Some(compute_hl2(high, low)),
            _ => return None,
        }
    }

    pub fn hlc3(&self) -> Option<f64> {
        let high = self.high();
        let low = self.low();
        let close = self.close();
        match (high, low, close) {
            (Some(high), Some(low), Some(close)) => return Some(compute_hlc3(high, low, close)),
            _ => return None,
        }
    }

    pub fn opens(&self) -> &[Option<f64>] {
        return self
            .asset_data_provider
            .get_opens(self.start_tick, self.end_tick);
    }

    pub fn highs(&self) -> &[Option<f64>] {
        return self
            .asset_data_provider
            .get_highs(self.start_tick, self.end_tick);
    }

    pub fn prev_high(&self, tick: usize) -> Option<f64> {
        if self.current_tick < tick {
            return None;
        }
        return self.asset_data_provider.get_high(self.current_tick - tick);
    }

    pub fn prev_low(&self, tick: usize) -> Option<f64> {
        if self.current_tick < tick {
            return None;
        }
        return self.asset_data_provider.get_low(self.current_tick - tick);
    }

    pub fn prev_close(&self, tick: usize) -> Option<f64> {
        if self.current_tick < tick {
            return None;
        }
        return self.asset_data_provider.get_close(self.current_tick - tick);
    }

    pub fn prev_highs(&self, length: usize) -> &[Option<f64>] {
        return self
            .asset_data_provider
            .get_highs(self.current_tick - (length - 1), self.current_tick);
    }

    pub fn prev_lows(&self, length: usize) -> &[Option<f64>] {
        return self
            .asset_data_provider
            .get_lows(self.current_tick - (length - 1), self.current_tick);
    }

    pub fn at_length(&self, length: usize) -> bool {
        return self.current_tick >= length - 1;
    }

    pub fn is_ohlcv_valid(&self) -> bool {
        return self.open().is_some()
            && self.high().is_some()
            && self.low().is_some()
            && self.close().is_some()
            && self.volume().is_some()
            && self.time().is_some();
    }
}
