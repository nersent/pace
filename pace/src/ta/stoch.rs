use crate::{
    common::window_cache::WindowCache,
    core::{context::Context, incremental::Incremental},
};

use super::bars::{highest, lowest};

/// Stochastic.
///
/// Similar to PineScript `ta.stoch(src, high, low, length)`, but `src` array requires to be truncated to the length and you need to keep track of the previous value of stoch.
pub fn stoch(
    value: Option<f64>,
    high: &[Option<f64>],
    low: &[Option<f64>],
    prev_stoch: Option<f64>,
) -> Option<f64> {
    value?;
    let high = highest(high);
    let low = lowest(low);

    if high.is_none() || low.is_none() {
        return None;
    }

    let diff = high.unwrap() - low.unwrap();

    if diff == 0.0 {
        return prev_stoch;
    }

    return Some(100.0 * (value.unwrap() - low.unwrap()) / diff);
}

/// Stochastic.
///
/// Same as PineScript `ta.stoch(src, high, low)`. Similar to `ta.stoch(src, high, low, length)`, but `length` is fixed and set on initialization.
pub struct Stoch {
    pub length: usize,
    pub ctx: Context,
    prev_stoch: Option<f64>,
    high_input_cache: WindowCache<Option<f64>>,
    low_input_cache: WindowCache<Option<f64>>,
}

impl Stoch {
    pub fn new(ctx: Context, length: usize) -> Self {
        assert!(length >= 1, "Stoch must have a length of at least 1");
        return Self {
            ctx: ctx.clone(),
            length,
            prev_stoch: None,
            high_input_cache: WindowCache::new(ctx.clone(), length),
            low_input_cache: WindowCache::new(ctx.clone(), length),
        };
    }
}

impl Incremental<(Option<f64>, Option<f64>, Option<f64>), Option<f64>> for Stoch {
    /// Input: `src, high, low`.
    fn next(&mut self, (value, high, low): (Option<f64>, Option<f64>, Option<f64>)) -> Option<f64> {
        self.high_input_cache.next(high);
        self.low_input_cache.next(low);

        if !self.ctx.bar.at_length(self.length) {
            return None;
        }

        let _stoch = stoch(
            value,
            self.high_input_cache.all(),
            self.low_input_cache.all(),
            self.prev_stoch,
        );
        self.prev_stoch = _stoch;

        return _stoch;
    }
}
