use std::{sync::Arc, time::Duration};

use polars::prelude::DataFrame;

use super::{data_provider::DataProvider, timeframe::Timeframe};

/// Implements `DataProvider`. Stores all data in memory.
pub struct InMemoryDataProvider {
    pub open: Vec<f64>,
    pub high: Vec<f64>,
    pub low: Vec<f64>,
    pub close: Vec<f64>,
    pub volume: Vec<f64>,
    pub time: Vec<Option<Duration>>,
    pub start_tick: usize,
    pub end_tick: usize,
    pub timeframe: Timeframe,
}

impl InMemoryDataProvider {
    pub fn with_timeframe(mut self, timeframe: Timeframe) -> Self {
        self.timeframe = timeframe;
        return self;
    }
}

impl DataProvider for InMemoryDataProvider {
    fn get_first_tick(&self) -> usize {
        return self.start_tick;
    }

    fn get_last_tick(&self) -> usize {
        return self.end_tick;
    }

    fn get_open(&self, index: usize) -> f64 {
        return self.open[index];
    }

    fn get_high(&self, index: usize) -> f64 {
        return self.high[index];
    }

    fn get_low(&self, index: usize) -> f64 {
        return self.low[index];
    }

    fn get_close(&self, index: usize) -> f64 {
        return self.close[index];
    }

    fn get_volume(&self, index: usize) -> f64 {
        return self.volume[index];
    }

    fn get_time(&self, index: usize) -> Option<Duration> {
        return self.time[index];
    }

    fn get_open_for_range(&self, start_index: usize, end_index: usize) -> &[f64] {
        return &self.open[start_index..end_index + 1];
    }

    fn get_high_for_range(&self, start_index: usize, end_index: usize) -> &[f64] {
        return &self.high[start_index..end_index + 1];
    }

    fn get_low_for_range(&self, start_index: usize, end_index: usize) -> &[f64] {
        return &self.low[start_index..end_index + 1];
    }

    fn get_close_for_range(&self, start_index: usize, end_index: usize) -> &[f64] {
        return &self.close[start_index..end_index + 1];
    }

    fn get_volume_for_range(&self, start_index: usize, end_index: usize) -> &[f64] {
        return &self.volume[start_index..end_index + 1];
    }

    fn find_tick(&self, seconds: u64) -> Option<usize> {
        for i in self.get_first_tick()..self.get_last_tick() {
            let time = self.get_time(i);
            let next_time = self.get_time(i + 1);

            if time.unwrap().as_secs() <= seconds && next_time.unwrap().as_secs() > seconds {
                return Some(i);
            }
        }

        return None;
    }

    fn get_timeframe(&self) -> Timeframe {
        return self.timeframe;
    }
}

impl InMemoryDataProvider {
    pub fn new(
        open: Vec<f64>,
        high: Vec<f64>,
        low: Vec<f64>,
        close: Vec<f64>,
        volume: Vec<f64>,
        time: Vec<Option<Duration>>,
    ) -> Self {
        let start_tick = 0;
        let end_tick = close.len() - 1;

        return Self {
            open,
            high,
            low,
            close,
            volume,
            start_tick,
            end_tick,
            time,
            timeframe: Timeframe::Unknown,
        };
    }

    pub fn from_values(values: Vec<f64>) -> Self {
        return Self {
            open: values.clone(),
            high: values.clone(),
            low: values.clone(),
            close: values.clone(),
            volume: values.clone(),
            start_tick: 0,
            end_tick: values.len() - 1,
            time: vec![None; values.len()],
            timeframe: Timeframe::Unknown,
        };
    }
}
