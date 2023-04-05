use crate::{
    common::float_series::FloatSeries,
    core::{context::Context, incremental::Incremental},
    utils::float::Float64Utils,
};

use super::simple_moving_average::Sma;

/// Standard Deviation.
///
/// Compared to `statistics::stdev`, this  calculates stdev based on a sliding window.
///
/// Same as PineScript `ta.stdev(src)`. Similar to `ta.stdev(src, length)`, but `length` is fixed and set on initialization.
pub struct Stdev {
    pub length: usize,
    pub ctx: Context,
    /// If `is_biased` is true, function will calculate using a biased estimate of the entire population, if false - unbiased estimate of a sample.
    pub is_biased: bool,
    sma: Sma,
    series: FloatSeries,
    _div_length: f64,
}

impl Stdev {
    // Biased by default.
    pub fn new(ctx: Context, length: usize, is_biased: bool) -> Self {
        assert!(length >= 1, "Stdev must have a length of at least 1");
        return Self {
            ctx: ctx.clone(),
            length,
            is_biased,
            sma: Sma::new(ctx.clone(), length),
            series: FloatSeries::new(ctx.clone()),
            _div_length: if is_biased {
                length as f64
            } else {
                (length - 1) as f64
            },
        };
    }

    fn compute_sum(fst: f64, snd: f64) -> f64 {
        let sum = fst + snd;
        if sum.compare_with_precision(0.0, 1e-10) {
            return 0.0;
        }
        return sum;
    }
}

impl Incremental<f64, f64> for Stdev {
    fn next(&mut self, value: f64) -> f64 {
        if !value.is_nan() {
            self.series.next(value);
        }

        let mean = -self.sma.next(value);

        if !self.series.is_filled(self.length) {
            return f64::NAN;
        }

        let mut sum = 0.0;

        for v in self.series.window(self.length) {
            sum += f64::powi(Self::compute_sum(*v, mean), 2);
        }

        let stdev = (sum / self._div_length).sqrt();

        return stdev;
    }
}
