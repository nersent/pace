use crate::{
    common::window_cache::WindowCache,
    core::{context::Context, incremental::Incremental},
};

/// Rate of Change. Calculates the percentage of change (rate of change) between the current value of `source` and its value `length` bars ago.
///
/// Same as PineScript `ta.roc(src)`. Similar to `ta.roc(src, length)`, but `length` is fixed and set on initialization.
pub struct Roc {
    pub length: usize,
    pub ctx: Context,
    input_cache: WindowCache<Option<f64>>,
}

impl Roc {
    pub fn new(ctx: Context, length: usize) -> Self {
        assert!(length >= 1, "Roc must have a length of at least 1");
        return Self {
            ctx: ctx.clone(),
            length,
            input_cache: WindowCache::new(ctx.clone(), length + 1),
        };
    }
}

impl Incremental<Option<f64>, Option<f64>> for Roc {
    fn next(&mut self, value: Option<f64>) -> Option<f64> {
        self.input_cache.next(value);

        let first_value = self.input_cache.first_unwrapped();
        let last_value = self.input_cache.last_unwrapped();
        let is_filled = self.input_cache.is_filled();

        if !is_filled || first_value.is_none() || last_value.is_none() {
            return None;
        }

        let first_value = first_value.unwrap();
        if first_value == 0.0 {
            return None;
        }
        let last_value = last_value.unwrap();
        return Some(100.0 * (last_value - first_value) / first_value);
    }
}
