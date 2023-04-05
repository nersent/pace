use crate::{
    common::float_series::FloatSeries,
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
    series: FloatSeries,
}

impl Dev {
    /// Biased by default.
    pub fn new(ctx: Context, length: usize) -> Self {
        assert!(length >= 1, "Dev must have a length of at least 1");
        return Self {
            ctx: ctx.clone(),
            length,
            sma: Sma::new(ctx.clone(), length),
            series: FloatSeries::new(ctx.clone()),
        };
    }
}

impl Incremental<f64, f64> for Dev {
    fn next(&mut self, value: f64) -> f64 {
        self.series.next(value);

        let mean = self.sma.next(value);

        if !self.series.is_filled(self.length) {
            return f64::NAN;
        }

        let mut sum = 0.0;

        for item in self.series.window(self.length) {
            sum += f64::abs(*item - mean);
        }

        let dev = sum / self.length as f64;

        return dev;
    }
}
