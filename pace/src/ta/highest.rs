use crate::{
    common::float_series::FloatSeries,
    core::{context::Context, incremental::Incremental},
};

use super::bars::highest;

/// Highest value for a given number of bars back.
///
/// Same as PineScript `ta.highest(src)`. Similar to `ta.highest(src, length)`, but `length` is fixed and set on initialization.
pub struct Highest {
    pub length: usize,
    pub ctx: Context,
    series: FloatSeries,
}

impl Highest {
    pub fn new(ctx: Context, length: usize) -> Self {
        assert!(length >= 1, "Highest must have a length of at least 1");
        return Self {
            ctx: ctx.clone(),
            length,
            series: FloatSeries::new(ctx.clone()),
        };
    }
}

impl Incremental<f64, f64> for Highest {
    // ctx.bar.high() by default
    fn next(&mut self, value: f64) -> f64 {
        self.series.next(value);

        if !self.series.is_filled(self.length) || value.is_nan() {
            return f64::NAN;
        }

        return highest(self.series.window(self.length));
    }
}
