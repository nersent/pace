use crate::{
    common::float_series::FloatSeries,
    core::{context::Context, incremental::Incremental},
    pinescript::common::PineScriptFloat64,
};

/// Rate of Change. Calculates the percentage of change (rate of change) between the current value of `source` and its value `length` bars ago.
///
/// Same as PineScript `ta.roc(src)`. Similar to `ta.roc(src, length)`, but `length` is fixed and set on initialization.
pub struct Roc {
    pub length: usize,
    pub ctx: Context,
    series: FloatSeries,
    _at_size: usize,
}

impl Roc {
    pub fn new(ctx: Context, length: usize) -> Self {
        assert!(length >= 1, "Roc must have a length of at least 1");
        return Self {
            length,
            ctx: ctx.clone(),
            series: FloatSeries::new(ctx.clone()),
            _at_size: length + 1,
        };
    }
}

impl Incremental<f64, f64> for Roc {
    fn next(&mut self, value: f64) -> f64 {
        self.series.next(value);

        if self.series.is_filled(self._at_size) {
            let back = self.series[self.length];
            return 100.0 * (value - back) / back;
        }

        return f64::NAN;
    }
}
