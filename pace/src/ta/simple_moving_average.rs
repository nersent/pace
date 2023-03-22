use crate::{
    common::{window_cache::WindowCache, window_validator::WindowValidator},
    core::{context::Context, incremental::Incremental},
};

/// Simple Moving Average. The sum of last y values of x, divided by y.
///
/// Same as PineScript `ta.sma(src)`. Similar to `ta.sma(src, length)`, but `length` is fixed and set on initialization.
pub struct Sma {
    pub length: usize,
    pub ctx: Context,
    _length_f64: f64,
    sum: f64,
    input_cache: WindowCache<Option<f64>>,
    batch_validator: WindowValidator,
}

impl Sma {
    pub fn new(ctx: Context, length: usize) -> Self {
        assert!(length >= 1, "Sma must have a length of at least 1");
        return Self {
            length,
            ctx: ctx.clone(),
            _length_f64: length as f64,
            sum: 0.0,
            input_cache: WindowCache::new(ctx.clone(), length),
            batch_validator: WindowValidator::new(ctx.clone(), length),
        };
    }
}

impl Incremental<Option<f64>, Option<f64>> for Sma {
    fn next(&mut self, value: Option<f64>) -> Option<f64> {
        if self.length == 1 {
            return value;
        }
        self.input_cache.next(value);
        let is_valid = self.batch_validator.next(value);
        let is_filled = self.input_cache.is_filled();
        let first_value = self.input_cache.first_unwrapped();
        let last_value = self.input_cache.last_unwrapped();
        let mut mean: Option<f64> = None;
        if let Some(last_value) = last_value {
            self.sum += last_value;
        }
        if is_filled && is_valid {
            mean = Some(self.sum / self._length_f64);
        }
        if let Some(first_value) = first_value {
            self.sum -= first_value;
        }
        return mean;
    }
}
