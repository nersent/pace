use crate::{
    common::window_cache::WindowCache,
    core::{context::Context, incremental::Incremental},
};

use super::simple_moving_average::Sma;

/// Deviation. Measure of difference between the series and it's `ta.sma`.
///
/// Same as PineScript `ta.dev(src)`. Similar to `ta.dev(src, length)`, but `length` is fixed and set on initialization.
pub struct Dev {
    pub length: usize,
    pub ctx: Context,
    sma: Sma,
    input_cache: WindowCache<Option<f64>>,
}

impl Dev {
    /// Biased by default.
    pub fn new(ctx: Context, length: usize) -> Self {
        assert!(length >= 1, "Dev must have a length of at least 1");
        return Self {
            ctx: ctx.clone(),
            length,
            sma: Sma::new(ctx.clone(), length),
            input_cache: WindowCache::new(ctx.clone(), length),
        };
    }
}

impl Incremental<Option<f64>, Option<f64>> for Dev {
    fn next(&mut self, value: Option<f64>) -> Option<f64> {
        if self.length == 1 {
            return Some(0.0);
        }

        self.input_cache.next(value);

        let mean = self.sma.next(value);

        if mean.is_none() || !self.input_cache.is_filled() {
            return None;
        }

        let mean = mean.unwrap();

        let values = self.input_cache.all();
        let sum = values
            .iter()
            .map(|v| (v.unwrap_or(mean) - mean).abs())
            .sum::<f64>();

        let dev = sum / self.length as f64;
        return Some(dev);
    }
}
