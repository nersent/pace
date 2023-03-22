use std::{
    borrow::{Borrow, BorrowMut},
    cell::{Cell, RefCell, RefMut, UnsafeCell},
    rc::Rc,
    sync::Arc,
    time::Duration,
};

use chrono::NaiveDateTime;

use super::data_provider::DataProvider;

pub struct Bar {
    ctx: Context,
    /// Current bar index. Numbering is zero-based, index of the first bar is 0, unless `start_tick` was set differently.
    ///
    /// Same as PineScript `bar_index`.
    pub index: usize,
    pub open: Option<f64>,
    pub high: Option<f64>,
    pub low: Option<f64>,
    pub close: Option<f64>,
    pub volume: Option<f64>,
}

impl Bar {
    pub fn new(ctx: Context) -> Self {
        let index = ctx.first_bar_index;
        let open = ctx.data.get_open(index);
        let high = ctx.data.get_high(index);
        let low = ctx.data.get_low(index);
        let close = ctx.data.get_close(index);
        let volume = ctx.data.get_volume(index);

        return Self {
            ctx,
            index,
            open,
            high,
            low,
            close,
            volume,
        };
    }

    /// Current time.
    ///
    /// Similar to PineScript `time`.
    pub fn time(&self) -> Option<Duration> {
        return self.ctx.data.get_time(self.index);
    }

    /// Current datetime.
    ///
    /// Similar to PineScript `time`.
    pub fn datetime(&self) -> Option<NaiveDateTime> {
        return self
            .time()
            .map(|time| NaiveDateTime::from_timestamp_millis(time.as_millis() as i64).unwrap());
    }

    /// Returns `true` if current bar is **green** (returns are positive).
    pub fn is_up(&self) -> bool {
        return self.close.unwrap() >= self.open.unwrap();
    }

    /// Checks if it's possible to perform calculations based on last `length` values.
    pub fn at_length(&self, length: usize) -> bool {
        return self.index >= length - 1;
    }
}

pub struct Context {
    pub data: Arc<dyn DataProvider + 'static + Send + Sync>,
    bar: Rc<UnsafeCell<Option<Bar>>>,
    // First bar index. Starts with 0, unless `start_tick` was set differently.
    pub first_bar_index: usize,
    /// Bar index of the last chart bar.
    ///
    /// Same as PineScript `last_bar_index`.
    pub last_bar_index: usize,
    // /// The total number of ticks between first and last bars.
    pub bars: usize,
}

/// Execution state across shared across all components.
impl Context {
    pub fn new(data: Arc<dyn DataProvider + 'static + Send + Sync>) -> Self {
        let first_bar_index = data.get_start_tick();
        let last_bar_index = data.get_end_tick();
        let bars = last_bar_index - first_bar_index + 1;
        return Self {
            data,
            first_bar_index,
            last_bar_index,
            bar: Rc::new(UnsafeCell::new(None)),
            bars,
        };
    }

    /// This creates a new instance of `Context`, but keeps all pointers to the same data, meaning you can deeply nest `Context` and keep the same state.
    pub fn clone(&self) -> Self {
        return Self {
            data: Arc::clone(&self.data),
            first_bar_index: self.first_bar_index,
            last_bar_index: self.last_bar_index,
            bars: self.bars,
            bar: Rc::clone(&self.bar),
        };
    }

    pub fn bar(&self) -> &Bar {
        unsafe { (*self.bar.get()).as_ref().unwrap() }
    }

    /// Returns **`N`** previous high price.
    pub fn high(&self, n: usize) -> Option<f64> {
        let tick = self.bar().index;
        if tick < n {
            return None;
        }
        return self.data.get_high(tick - n);
    }

    // /// Returns **`N`** previous low price.
    pub fn low(&self, n: usize) -> Option<f64> {
        let tick = self.bar().index;
        if tick < n {
            return None;
        }
        return self.data.get_low(tick - n);
    }

    // /// Returns **`N`** previous open price.
    pub fn close(&self, n: usize) -> Option<f64> {
        let tick = self.bar().index;
        if tick < n {
            return None;
        }
        return self.data.get_close(tick - n);
    }

    // /// Returns **`N`** previous volume.
    pub fn volume(&self, n: usize) -> Option<f64> {
        let tick = self.bar().index;
        if tick < n {
            return None;
        }
        return self.data.get_volume(tick - n);
    }

    // /// Returns a list of **`N`** previous open prices.
    pub fn opens(&self, length: usize) -> &[Option<f64>] {
        let tick = self.bar().index;
        return self.data.get_open_for_range(tick - (length - 1), tick);
    }

    // /// Returns a list of **`N`** previous high prices.
    pub fn highs(&self, length: usize) -> &[Option<f64>] {
        let tick = self.bar().index;
        return self.data.get_high_for_range(tick - (length - 1), tick);
    }

    // /// Returns a list of **`N`** previous low prices.
    pub fn lows(&self, length: usize) -> &[Option<f64>] {
        let tick = self.bar().index;
        return self.data.get_low_for_range(tick - (length - 1), tick);
    }

    // /// Returns a list of **`N`** previous close prices.
    pub fn closes(&self, length: usize) -> &[Option<f64>] {
        let tick = self.bar().index;
        return self.data.get_close_for_range(tick - (length - 1), tick);
    }

    // /// Returns a list of **`N`** previous volumes.
    pub fn volumes(&self, length: usize) -> &[Option<f64>] {
        let tick = self.bar().index;
        return self.data.get_volume_for_range(tick - (length - 1), tick);
    }
}

impl Iterator for Context {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if let Some(bar) = &mut *self.bar.get() {
                if bar.index >= self.last_bar_index {
                    return None;
                }
                bar.index += 1;
                bar.open = self.data.get_open(bar.index);
                bar.high = self.data.get_high(bar.index);
                bar.low = self.data.get_low(bar.index);
                bar.close = self.data.get_close(bar.index);
                bar.volume = self.data.get_volume(bar.index);
                return Some(bar.index);
            } else {
                *self.bar.get() = Some(Bar::new(self.clone()));
                return Some(self.first_bar_index);
            }
        }
    }
}
