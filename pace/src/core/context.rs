use std::{
    borrow::{Borrow, BorrowMut},
    cell::{Cell, Ref, RefCell, RefMut, UnsafeCell},
    rc::Rc,
    sync::Arc,
    time::Duration,
};

use chrono::NaiveDateTime;

use crate::utils::{math::round_to_min_tick, time::to_datetime};

use super::data_provider::{AnyDataProvider, DataProvider};

pub struct Bar {
    pub index: Rc<Cell<usize>>,
    pub data: AnyDataProvider,
}

impl Bar {
    /// Current bar index. Numbering is zero-based, index of the first bar is 0, unless `start_tick` was set differently.
    ///
    /// Same as PineScript `bar_index`.
    pub fn index(&self) -> usize {
        return self.index.get();
    }

    /// Current time.
    ///
    /// Similar to PineScript `time`.
    pub fn time(&self) -> Option<Duration> {
        return self.data.get_time(self.index.get());
    }

    /// Current datetime.
    ///
    /// Similar to PineScript `time`.
    pub fn datetime(&self) -> Option<NaiveDateTime> {
        return self.time().map(|time| to_datetime(time.as_millis() as i64));
    }

    /// Returns `true` if current bar is **green** (returns are positive).
    pub fn is_up(&self) -> bool {
        return self.close() >= self.open();
    }

    /// Checks if it's possible to perform calculations based on last `length` values.
    pub fn at_length(&self, length: usize) -> bool {
        return self.index.get() + 1 >= length; //
    }

    /// Same as PineScript `open`.
    pub fn open(&self) -> f64 {
        return self.data.get_open(self.index.get());
    }

    /// Same as PineScript `high`.
    pub fn high(&self) -> f64 {
        return self.data.get_high(self.index.get());
    }

    /// Same as PineScript `low`.
    pub fn low(&self) -> f64 {
        return self.data.get_low(self.index.get());
    }

    /// Same as PineScript `close`.
    pub fn close(&self) -> f64 {
        return self.data.get_close(self.index.get());
    }

    /// Same as PineScript `volume`.
    pub fn volume(&self) -> f64 {
        return self.data.get_volume(self.index.get());
    }
}

pub struct Context {
    pub data: AnyDataProvider,
    pub bar: Bar,
    // First bar index. Starts with 0, unless `start_tick` was set differently.
    pub first_bar_index: usize,
    /// Bar index of the last chart bar.
    /// Same as PineScript `last_bar_index`.
    pub last_bar_index: usize,
    /// The total number of ticks between first and last bars.
    pub bars: usize,
    is_running: Rc<Cell<bool>>,
}

/// Execution state across shared across all components.
impl Context {
    pub fn new(data: AnyDataProvider) -> Self {
        let first_bar_index = data.get_first_tick();
        let last_bar_index = data.get_last_tick();
        let bars = last_bar_index - first_bar_index + 1;

        let bar = Bar {
            data: Arc::clone(&data),
            index: Rc::new(Cell::new(first_bar_index)),
        };

        return Self {
            data,
            first_bar_index,
            last_bar_index,
            bar,
            bars,
            is_running: Rc::new(Cell::new(false)),
        };
    }

    /// This creates a new instance of `Context`, but keeps all pointers to the same data, meaning you can deeply nest `Context` and keep the same state.
    pub fn clone(&self) -> Self {
        return Self {
            data: Arc::clone(&self.data),
            first_bar_index: self.first_bar_index,
            last_bar_index: self.last_bar_index,
            bars: self.bars,
            bar: Bar {
                index: Rc::clone(&self.bar.index),
                data: Arc::clone(&self.data),
            },
            is_running: Rc::clone(&self.is_running),
        };
    }

    /// Rounds `size` to the nearest multiple of the minimum order quantity.
    pub fn round_contracts(&self, size: f64) -> f64 {
        if let Some(sym_info) = self.data.get_sym_info() {
            if sym_info.min_qty.is_nan() {
                return size;
            }
            // 0.000001
            let round_val = 1000000.0;
            return ((size * round_val) + f64::EPSILON).round() / round_val;
            // return (size * round_val).floor() / round_val;
        }
        return size;
    }

    /// Checks if `size` is a valid order quantity by comparing it to the minimum order quantity.
    pub fn validate_contracts(&self, size: f64) -> bool {
        if let Some(sym_info) = self.data.get_sym_info() {
            return sym_info.min_qty.is_nan() || size >= sym_info.min_qty;
        }
        return true;
    }

    pub fn round_to_min_tick(&self, value: f64) -> f64 {
        return self
            .data
            .get_sym_info()
            .map(|sym_info| round_to_min_tick(value, sym_info.min_tick))
            .unwrap_or(value);
    }

    /// Returns **`N`** previous high price.
    pub fn high(&self, n: usize) -> f64 {
        let tick = self.bar.index.get();
        if tick < n {
            return f64::NAN;
        }
        return self.data.get_high(tick - n);
    }

    /// Returns **`N`** previous low price.
    pub fn low(&self, n: usize) -> f64 {
        let tick = self.bar.index.get();
        if tick < n {
            return f64::NAN;
        }
        return self.data.get_low(tick - n);
    }

    /// Returns **`N`** previous open price.
    pub fn close(&self, n: usize) -> f64 {
        let tick = self.bar.index.get();
        if tick < n {
            return f64::NAN;
        }
        return self.data.get_close(tick - n);
    }

    /// Returns **`N`** previous volume.
    pub fn volume(&self, n: usize) -> f64 {
        let tick = self.bar.index.get();
        if tick < n {
            return f64::NAN;
        }
        return self.data.get_volume(tick - n);
    }

    /// Returns a list of **`N`** previous open prices.
    pub fn opens(&self, length: usize) -> &[f64] {
        let tick = self.bar.index.get();
        return self.data.get_open_for_range(tick - (length - 1), tick);
    }

    /// Returns a list of **`N`** previous high prices.
    pub fn highs(&self, length: usize) -> &[f64] {
        let tick = self.bar.index.get();
        return self.data.get_high_for_range(tick - (length - 1), tick);
    }

    /// Returns a list of **`N`** previous low prices.
    pub fn lows(&self, length: usize) -> &[f64] {
        let tick = self.bar.index.get();
        return self.data.get_low_for_range(tick - (length - 1), tick);
    }

    /// Returns a list of **`N`** previous close prices.
    pub fn closes(&self, length: usize) -> &[f64] {
        let tick = self.bar.index.get();
        return self.data.get_close_for_range(tick - (length - 1), tick);
    }

    /// Returns a list of **`N`** previous volumes.
    pub fn volumes(&self, length: usize) -> &[f64] {
        let tick = self.bar.index.get();
        return self.data.get_volume_for_range(tick - (length - 1), tick);
    }
}

impl Iterator for Context {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.is_running.get() {
            self.is_running.set(true);
            return Some(self.first_bar_index);
        }

        let current_index = self.bar.index.get() + 1;
        self.bar.index.set(current_index);

        if current_index <= self.last_bar_index {
            return Some(current_index);
        }

        return None;
    }
}
