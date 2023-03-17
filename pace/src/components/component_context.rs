use std::{
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

/// Provides data for all components.
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

    fn get_execution_state(&mut self) -> RefMut<ComponentExecutionState> {
        return self.execution_state.as_ref().borrow_mut();
    }

    /// This creates a new instance of `ComponentContext`, but keeps all pointers to the same data, meaning you can deeply nest `ComponentContext` and keep the same state.
    pub fn clone(&self) -> Self {
        return Self {
            data: Arc::clone(&self.data),
            execution_state: Rc::clone(&self.execution_state),
        };
    }

    /// Counts the total number of ticks between first and last bars.
    pub fn ticks(&self) -> usize {
        return self.last_bar_index() - self.first_bar_index() + 1;
    }

    /// Checks if it's possible to perform calculations based on last `length` values.
    pub fn at_length(&self, length: usize) -> bool {
        return self.bar_index() >= length - 1;
    }

    /// Current bar index. Numbering is zero-based, index of the first bar is 0, unless `start_tick` was set differently.
    ///
    /// Same as PineScript `bar_index`.
    pub fn bar_index(&self) -> usize {
        return self.execution_state.borrow().current_tick;
    }

    /// First bar index. Starts with 0, unless `start_tick` was set differently.
    pub fn first_bar_index(&self) -> usize {
        return self.execution_state.borrow().start_tick;
    }

    /// Bar index of the last chart bar.
    ///
    /// Same as PineScript `last_bar_index`.
    pub fn last_bar_index(&self) -> usize {
        return self.execution_state.borrow().end_tick;
    }

    /// Returns `true` if current bar is **green** (returns are positive).
    pub fn is_up(&self) -> bool {
        let open = self.open().unwrap();
        let close = self.close().unwrap();
        return close >= open;
    }

    /// Current open price.
    ///
    /// Same as PineScript `open`.
    pub fn open(&self) -> Option<f64> {
        return self.data.get_open(self.bar_index());
    }

    /// Current high price.
    ///
    /// Same as PineScript `high`.
    pub fn high(&self) -> Option<f64> {
        return self.data.get_high(self.bar_index());
    }

    /// Current low price.
    ///
    /// Same as PineScript `low`.
    pub fn low(&self) -> Option<f64> {
        return self.data.get_low(self.bar_index());
    }

    /// Current close price.
    ///   
    /// Same as PineScript `close`.
    pub fn close(&self) -> Option<f64> {
        return self.data.get_close(self.bar_index());
    }

    /// Current volume.
    ///
    /// Same as PineScript `volume`.
    pub fn volume(&self) -> Option<f64> {
        return self.data.get_volume(self.bar_index());
    }

    /// Current time.
    ///
    /// Similar to PineScript `time`.
    pub fn time(&self) -> Option<Duration> {
        return self.data.get_time(self.bar_index());
    }

    /// Current datetime.
    ///
    /// Similar to PineScript `time`.
    pub fn datetime(&self) -> Option<NaiveDateTime> {
        return self
            .time()
            .map(|time| NaiveDateTime::from_timestamp_millis(time.as_millis() as i64).unwrap());
    }

    /// Returns **`N`** previous high price.
    pub fn prev_high(&self, n: usize) -> Option<f64> {
        if self.bar_index() < n {
            return None;
        }
        return self.data.get_high(self.bar_index() - n);
    }

    /// Returns **`N`** previous low price.
    pub fn prev_low(&self, n: usize) -> Option<f64> {
        if self.bar_index() < n {
            return None;
        }
        return self.data.get_low(self.bar_index() - n);
    }

    /// Returns **`N`** previous open price.
    pub fn prev_close(&self, n: usize) -> Option<f64> {
        if self.bar_index() < n {
            return None;
        }
        return self.data.get_close(self.bar_index() - n);
    }

    /// Returns **`N`** previous volume.
    pub fn prev_volume(&self, n: usize) -> Option<f64> {
        if self.bar_index() < n {
            return None;
        }
        return self.data.get_volume(self.bar_index() - n);
    }

    /// Returns a list of **`N`** previous open prices.
    pub fn prev_opens(&self, length: usize) -> &[Option<f64>] {
        let tick = self.bar_index();
        return self.data.get_open_for_range(tick - (length - 1), tick);
    }

    /// Returns a list of **`N`** previous high prices.
    pub fn prev_highs(&self, length: usize) -> &[Option<f64>] {
        let tick = self.bar_index();
        return self.data.get_high_for_range(tick - (length - 1), tick);
    }

    /// Returns a list of **`N`** previous low prices.
    pub fn prev_lows(&self, length: usize) -> &[Option<f64>] {
        let tick = self.bar_index();
        return self.data.get_low_for_range(tick - (length - 1), tick);
    }

    /// Returns a list of **`N`** previous close prices.
    pub fn prev_closes(&self, length: usize) -> &[Option<f64>] {
        let tick = self.bar_index();
        return self.data.get_close_for_range(tick - (length - 1), tick);
    }

    /// Returns a list of **`N`** previous volumes.
    pub fn prev_volumes(&self, length: usize) -> &[Option<f64>] {
        let tick = self.bar_index();
        return self.data.get_volume_for_range(tick - (length - 1), tick);
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
