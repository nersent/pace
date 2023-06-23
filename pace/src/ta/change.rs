use crate::{
    common::float_series::FloatSeries,
    core::{context::Context, incremental::Incremental},
};

/// Compares the current `source` value to its value `length` bars ago and returns the difference.
///
/// Same as PineScript `ta.change(src)`. Similar to `ta.change(src, length)`, but `length` is fixed and set on initialization.
pub struct Change {
    pub length: usize,
    pub ctx: Context,
    series: FloatSeries,
    _at_size: usize,
}

impl Change {
    /// Length is `1` by default.
    pub fn new(ctx: Context, length: usize) -> Self {
        assert!(length >= 1, "Change must have a length of at least 1");
        return Self {
            ctx: ctx.clone(),
            length,
            series: FloatSeries::new(ctx.clone()),
            _at_size: length + 1,
        };
    }
}

impl Incremental<f64, f64> for Change {
    fn next(&mut self, value: f64) -> f64 {
        self.series.next(value);

        if !self.series.is_filled(self._at_size) {
            return f64::NAN;
        }

        return value - self.series[self.length];
    }
}
