use crate::core::{context::Context, incremental::Incremental};

use super::running_moving_average::Rma;

/// Relative Strength Index. It is calculated using the Running Moving Average of upward and downward changes of `source` over the last `length` bars.
///
/// Same as PineScript `ta.rsi(src)`. Similar to `ta.rsi(src, length)`, but `length` is fixed and set on initialization.
pub struct Rsi {
    pub length: usize,
    pub ctx: Context,
    up_rma: Rma,
    down_rma: Rma,
    prev_input_value: f64,
}

impl Rsi {
    pub fn new(ctx: Context, length: usize) -> Self {
        assert!(length >= 2, "Rsi must have a length of at least 2");
        return Self {
            length,
            ctx: ctx.clone(),
            prev_input_value: f64::NAN,
            up_rma: Rma::new(ctx.clone(), length),
            down_rma: Rma::new(ctx.clone(), length),
        };
    }
}

impl Incremental<f64, f64> for Rsi {
    fn next(&mut self, value: f64) -> f64 {
        let change = value - self.prev_input_value;

        let (up_change, down_change) = if change.is_nan() {
            (f64::NAN, f64::NAN)
        } else {
            (f64::max(change, 0.0), -f64::min(change, 0.0))
        };

        let up = self.up_rma.next(up_change);
        let down = self.down_rma.next(down_change);

        let rs = up / down;
        let rsi = 100.0 - 100.0 / (1.0 + rs);

        self.prev_input_value = value;

        return rsi;
    }
}
