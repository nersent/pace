use crate::{
    common::float_series::FloatSeries,
    core::{context::Context, incremental::Incremental},
    pinescript::common::PineScriptFloat64,
    utils::float::Float64Utils,
};

/// Simple Moving Average. The sum of last y values of x, divided by y.
///
/// Same as PineScript `ta.sma(src)`. Similar to `ta.sma(src, length)`, but `length` is fixed and set on initialization.
pub struct Sma {
    pub length: usize,
    pub ctx: Context,
    _length_f64: f64,
    _first_item_length: usize,
    sum: f64,
    series: FloatSeries,
    mean: f64,
}

impl Sma {
    pub fn new(ctx: Context, length: usize) -> Self {
        assert!(length >= 1, "Sma must have a length of at least 1");
        return Self {
            length,
            ctx: ctx.clone(),
            _length_f64: length as f64,
            _first_item_length: length - 1,
            sum: 0.0,
            series: FloatSeries::new(ctx.clone()),
            mean: f64::NAN,
        };
    }
}

impl Incremental<f64, f64> for Sma {
    fn next(&mut self, value: f64) -> f64 {
        if !value.is_nan() {
            self.series.next(value);

            self.sum += value;

            if self.series.is_filled(self.length) {
                self.mean = self.sum / self._length_f64;
                self.sum -= self.series[self._first_item_length];
            }
        }

        return self.mean;
    }
}
