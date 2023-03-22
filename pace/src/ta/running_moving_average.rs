use crate::core::{context::Context, incremental::Incremental};

use super::exponential_moving_average::Ema;

/// Running Moving Average. Used in RSI. It is the exponentially weighted moving average with alpha = 1 / length.
///
/// Same as PineScript `ta.rma(src)`. Similar to `ta.rma(src, length)`, but `length` is fixed and set on initialization.
pub struct Rma {
    pub length: usize,
    pub ctx: Context,
    ema: Ema,
}

impl Rma {
    pub fn new(ctx: Context, length: usize) -> Self {
        assert!(length >= 1, "Rma must have a length of at least 1");
        return Self {
            length,
            ctx: ctx.clone(),
            ema: Ema::with_alpha(ctx.clone(), length, 1.0 / length as f64),
        };
    }
}

impl Incremental<Option<f64>, Option<f64>> for Rma {
    fn next(&mut self, value: Option<f64>) -> Option<f64> {
        return self.ema.next(value);
    }
}
