use std::{rc::Rc, time::Duration};

use crate::data::asset_data_provider::AssetDataProvider;

pub struct ExecutionContext {
    pub asset_data_provider: Rc<dyn AssetDataProvider + 'static>,
    pub current_tick: usize,
    start_tick: usize,
    end_tick: usize,
    is_running: bool,
}

impl ExecutionContext {
    pub fn new(
        asset_data_provider: Rc<dyn AssetDataProvider + 'static>,
        start_tick: usize,
        end_tick: usize,
    ) -> Self {
        return ExecutionContext {
            current_tick: start_tick,
            start_tick,
            end_tick,
            asset_data_provider,
            is_running: false,
        };
    }

    pub fn from_asset(asset_data_provider: Rc<dyn AssetDataProvider + 'static>) -> Self {
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
}
