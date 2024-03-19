use super::bars::lowest;
use crate::{
    common::float_series::FloatSeries,
    core::{context::Context, incremental::Incremental},
};

/// Lowest value for a given number of bars back.
///
/// Same as PineScript `ta.lowest(src)`. Similar to `ta.lowest(src, length)`, but `length` is fixed and set on initialization.
pub struct Lowest {
    pub length: usize,
    pub ctx: Context,
    series: FloatSeries,
}

impl Lowest {
    pub fn new(ctx: Context, length: usize) -> Self {
        assert!(length >= 1, "Lowest must have a length of at least 1");
        return Self {
            ctx: ctx.clone(),
            length,
            series: FloatSeries::new(ctx.clone()),
        };
    }
}

impl Incremental<f64, f64> for Lowest {
    // ctx.bar.low() by default
    fn next(&mut self, value: f64) -> f64 {
        self.series.next(value);

        if !self.series.is_filled(self.length) || value.is_nan() {
            return f64::NAN;
        }

        return lowest(self.series.window(self.length));
    }
}
