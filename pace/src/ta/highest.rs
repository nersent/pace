use crate::{
    common::window_cache::WindowCache,
    core::{context::Context, incremental::Incremental},
};

use super::bars::highest;

/// Highest value for a given number of bars back.
///
/// Same as PineScript `ta.highest(src)`. Similar to `ta.highest(src, length)`, but `length` is fixed and set on initialization.
pub struct Highest {
    pub length: usize,
    pub ctx: Context,
    input_cache: WindowCache<Option<f64>>,
}

impl Highest {
    pub fn new(ctx: Context, length: usize) -> Self {
        assert!(length >= 1, "Highest must have a length of at least 1");
        return Self {
            ctx: ctx.clone(),
            length,
            input_cache: WindowCache::new(ctx.clone(), length),
        };
    }
}

impl Incremental<Option<f64>, Option<f64>> for Highest {
    fn next(&mut self, value: Option<f64>) -> Option<f64> {
        self.input_cache.next(value);

        if !self.ctx.bar.at_length(self.length) {
            return None;
        }

        return highest(self.input_cache.all());
    }
}
