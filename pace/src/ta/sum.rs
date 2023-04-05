use crate::{
    common::float_series::FloatSeries,
    core::{context::Context, incremental::Incremental},
    pinescript::common::PineScriptFloat64,
};

/// The sum function returns the sliding sum of last y values of x.
///
/// Same as PineScript `math.sum(src)`. Similar to `math.sum(src, length)`, but `length` is fixed and set on initialization.
pub struct Sum {
    pub length: usize,
    pub ctx: Context,
    sum: f64,
    _sum: f64,
    series: FloatSeries,
    _first_item_length: usize,
}

impl Sum {
    pub fn new(ctx: Context, length: usize) -> Self {
        assert!(length >= 1, "Sum must have a length of at least 1");
        return Self {
            ctx: ctx.clone(),
            length,
            series: FloatSeries::new(ctx.clone()),
            _first_item_length: length - 1,
            sum: 0.0,
            _sum: f64::NAN,
        };
    }
}

impl Incremental<f64, f64> for Sum {
    fn next(&mut self, value: f64) -> f64 {
        if !value.is_nan() {
            self.series.next(value);

            self.sum += value;

            if self.series.is_filled(self.length) {
                self._sum = self.sum;
                self.sum -= self.series[self._first_item_length];
            }
        }

        return self._sum;
    }
}
