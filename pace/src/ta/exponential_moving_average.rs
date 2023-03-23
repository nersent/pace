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
    prev_value: Option<f64>,
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
            prev_value: None,
        };
    }
}

impl Incremental<Option<f64>, Option<f64>> for Ema {
    fn next(&mut self, value: Option<f64>) -> Option<f64> {
        if self.length == 1 {
            return value;
        }
        if !self.ctx.bar.at_length(self.length - 1) {
            self.sma.next(value);
            return None;
        }
        match self.prev_value {
            Some(prev_value) => {
                let ema = self.alpha * value.unwrap() + (1.0 - self.alpha) * prev_value;
                self.prev_value = Some(ema);
                return self.prev_value;
            }
            None => {
                self.prev_value = self.sma.next(value);
                return self.prev_value;
            }
        }
    }
}
