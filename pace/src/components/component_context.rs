use std::{
    borrow::BorrowMut,
    cell::{RefCell, RefMut},
    rc::Rc,
    sync::Arc,
    time::Duration,
};

use chrono::NaiveDateTime;

use crate::data::data_provider::DataProvider;

pub struct ComponentExecutionState {
    pub current_tick: usize,
    pub is_running: bool,
    pub start_tick: usize,
    pub end_tick: usize,
}

impl ComponentExecutionState {
    pub fn new(start_tick: usize, end_tick: usize) -> Self {
        return Self {
            current_tick: 0,
            is_running: false,
            start_tick,
            end_tick,
        };
    }

    pub fn next(&mut self) -> bool {
        if !self.is_running {
            self.is_running = true;
            return true;
        }
        self.current_tick += 1;
        return self.current_tick <= self.end_tick;
    }
}

pub struct ComponentContext {
    pub data: Arc<dyn DataProvider + 'static + Send + Sync>,
    execution_state: Rc<RefCell<ComponentExecutionState>>,
}

/// Shared within all components.
impl ComponentContext {
    pub fn from_data_provider(
        data_provider: Arc<dyn DataProvider + 'static + Send + Sync>,
    ) -> Self {
        let start_tick = data_provider.get_start_tick();
        let end_tick = data_provider.get_end_tick();
        return Self {
            data: data_provider,
            execution_state: Rc::new(RefCell::new(ComponentExecutionState::new(
                start_tick, end_tick,
            ))),
        };
    }

    pub fn clone(&self) -> Self {
        return Self {
            data: Arc::clone(&self.data),
            execution_state: Rc::clone(&self.execution_state),
        };
    }

    fn get_execution_state(&mut self) -> RefMut<ComponentExecutionState> {
        return self.execution_state.as_ref().borrow_mut();
    }

    pub fn ticks(&self) -> usize {
        return self.last_bar_index() - self.first_bar_index() + 1;
    }

    pub fn at_length(&self, length: usize) -> bool {
        return self.bar_index() >= length - 1;
    }

    pub fn bar_index(&self) -> usize {
        return self.execution_state.borrow().current_tick;
    }

    pub fn first_bar_index(&self) -> usize {
        return self.execution_state.borrow().start_tick;
    }

    pub fn last_bar_index(&self) -> usize {
        return self.execution_state.borrow().end_tick;
    }

    /// Indicates that current bar is green
    pub fn is_up(&self) -> bool {
        let open = self.open().unwrap();
        let close = self.close().unwrap();
        return close >= open;
    }

    pub fn open(&self) -> Option<f64> {
        return self.data.get_open(self.bar_index());
    }

    pub fn high(&self) -> Option<f64> {
        return self.data.get_high(self.bar_index());
    }

    pub fn low(&self) -> Option<f64> {
        return self.data.get_low(self.bar_index());
    }

    pub fn close(&self) -> Option<f64> {
        return self.data.get_close(self.bar_index());
    }

    pub fn volume(&self) -> Option<f64> {
        return self.data.get_volume(self.bar_index());
    }

    pub fn time(&self) -> Option<Duration> {
        return self.data.get_time(self.bar_index());
    }

    pub fn datetime(&self) -> Option<NaiveDateTime> {
        return self
            .time()
            .map(|time| NaiveDateTime::from_timestamp_millis(time.as_millis() as i64).unwrap());
    }

    pub fn prev_high(&self, bar: usize) -> Option<f64> {
        if self.bar_index() < bar {
            return None;
        }
        return self.data.get_high(self.bar_index() - bar);
    }

    pub fn prev_low(&self, bar: usize) -> Option<f64> {
        if self.bar_index() < bar {
            return None;
        }
        return self.data.get_low(self.bar_index() - bar);
    }

    pub fn prev_close(&self, bar: usize) -> Option<f64> {
        if self.bar_index() < bar {
            return None;
        }
        return self.data.get_close(self.bar_index() - bar);
    }

    pub fn prev_volume(&self, bar: usize) -> Option<f64> {
        if self.bar_index() < bar {
            return None;
        }
        return self.data.get_volume(self.bar_index() - bar);
    }

    pub fn prev_highs(&self, length: usize) -> &[Option<f64>] {
        let tick = self.bar_index();
        return self.data.get_high_for_range(tick - (length - 1), tick);
    }

    pub fn prev_lows(&self, length: usize) -> &[Option<f64>] {
        let tick = self.bar_index();
        return self.data.get_low_for_range(tick - (length - 1), tick);
    }

    pub fn lowest_price(&self) -> Option<f64> {
        let open = self.open();
        let close = self.close();
        let high = self.high();
        let low = self.low();

        let lowest_price = low
            .unwrap()
            .min(close.unwrap())
            .min(open.unwrap())
            .min(high.unwrap());

        return Some(lowest_price);
    }

    pub fn highest_price(&self) -> Option<f64> {
        let open = self.open();
        let close = self.close();
        let high = self.high();
        let low = self.low();

        let highest_price = high
            .unwrap()
            .max(close.unwrap())
            .max(open.unwrap())
            .max(low.unwrap());

        return Some(highest_price);
    }
}

impl Iterator for ComponentContext {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let mut state = self.get_execution_state();
        if state.next() {
            return Some(state.current_tick);
        }
        return None;
    }
}
