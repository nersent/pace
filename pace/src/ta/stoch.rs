use crate::{
    common::float_series::FloatSeries,
    core::{context::Context, incremental::Incremental},
    pinescript::common::PineScriptFloat64,
    utils::float::Float64Utils,
};

use super::bars::{highest, lowest};

/// Stochastic.
///
/// Similar to PineScript `ta.stoch(src, high, low, length)`, but `src` array requires to be truncated to the length and you need to keep track of the previous value of stoch.
pub fn stoch(value: f64, high: &[f64], low: &[f64], prev_stoch: f64) -> f64 {
    let high = highest(high);
    let low = lowest(low);

    let diff = high - low;

    if diff.is_zero() || value.is_nan() || low.is_nan() || high.is_nan() {
        return prev_stoch;
    }

    return 100.0 * (value - low) / diff;
}

/// Stochastic.
///
/// Same as PineScript `ta.stoch(src, high, low)`. Similar to `ta.stoch(src, high, low, length)`, but `length` is fixed and set on initialization.
pub struct Stoch {
    pub length: usize,
    pub ctx: Context,
    prev_stoch: f64,
    high_series: FloatSeries,
    low_series: FloatSeries,
}

impl Stoch {
    pub fn new(ctx: Context, length: usize) -> Self {
        assert!(length >= 1, "Stoch must have a length of at least 1");
        return Self {
            ctx: ctx.clone(),
            length,
            prev_stoch: f64::NAN,
            high_series: FloatSeries::new(ctx.clone()),
            low_series: FloatSeries::new(ctx.clone()),
        };
    }
}

impl Incremental<(f64, f64, f64), f64> for Stoch {
    /// Input: `src, high, low`.
    fn next(&mut self, (value, high, low): (f64, f64, f64)) -> f64 {
        self.high_series.next(high);
        self.low_series.next(low);

        if !self.high_series.is_filled(self.length) {
            return f64::NAN;
        }

        let _stoch = stoch(
            value,
            self.high_series.window(self.length),
            self.low_series.window(self.length),
            self.prev_stoch,
        );
        self.prev_stoch = _stoch;

        return _stoch;
    }
}
