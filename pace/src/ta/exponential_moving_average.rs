use crate::core::{context::Context, incremental::Incremental};

use super::simple_moving_average::Sma;

/// Exponential Moving Vverage. Weighting factors decrease exponentially.
///
/// Same as PineScript `ta.ema(src)`. Similar to `ta.ema(src, length)`, but `length` is fixed and set on initialization.
pub struct Ema {
    pub alpha: f64,
    pub length: usize,
    pub ctx: Context,
    sma: Sma,
    prev_value: f64,
    at_length: usize,
    alpha_mult: f64,
}

impl Ema {
    pub fn new(ctx: Context, length: usize) -> Self {
        return Self::with_alpha(ctx, length, 2.0 / (length as f64 + 1.0));
    }

    pub fn with_alpha(ctx: Context, length: usize, alpha: f64) -> Self {
        assert!(length >= 1, "Ema must have a length of at least 1");
        return Self {
            length,
            alpha,
            ctx: ctx.clone(),
            sma: Sma::new(ctx.clone(), length),
            at_length: length + 1,
            prev_value: f64::NAN,
            alpha_mult: 1.0 - alpha,
        };
    }
}

impl Incremental<f64, f64> for Ema {
    fn next(&mut self, value: f64) -> f64 {
        if !self.ctx.bar.at_length(self.at_length) || self.prev_value.is_nan() {
            self.prev_value = self.sma.next(value);
            return self.prev_value;
        }

        if value.is_nan() {
            return f64::NAN;
        }

        self.prev_value = self.alpha * value + self.alpha_mult * self.prev_value;
        return self.prev_value;
    }
}
