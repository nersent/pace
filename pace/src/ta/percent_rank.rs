use crate::{
    common::float_series::FloatSeries,
    core::{context::Context, incremental::Incremental},
};

/// Percent rank is the percents of how many previous values was less than or equal to the current value of given series.
///
/// Same as PineScript `ta.percentrank(src)`. Similar to `ta.percentrank(src, length)`, but `length` is fixed and set on initialization.
pub struct Prank {
    pub length: usize,
    pub ctx: Context,
    series: FloatSeries,
}

impl Prank {
    pub fn new(ctx: Context, length: usize) -> Self {
        assert!(length >= 1, "Prank must have a length of at least 1");
        return Self {
            ctx: ctx.clone(),
            length,
            series: FloatSeries::new(ctx.clone()),
        };
    }
}

impl Incremental<f64, f64> for Prank {
    fn next(&mut self, value: f64) -> f64 {
        let mut prank = f64::NAN;

        if self.series.is_filled(self.length) {
            let mut count: f64 = 0.0;

            for item in self.series.window(self.length) {
                if !item.is_nan() && *item <= value {
                    count += 1.0;
                }
            }

            prank = count / self.length as f64 * 100.0;
        }

        self.series.next(value);

        return prank;
    }
}
