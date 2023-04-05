use crate::{
    common::float_series::FloatSeries,
    core::{context::Context, incremental::Incremental},
};

use super::bars::highest_bars;

/// Highest value offset for a given number of bars back.
///
/// Same as PineScript `ta.highestbars(src)`. Similar to `ta.highestbars(src, length)`, but `length` is fixed and set on initialization.
pub struct HighestBars {
    pub length: usize,
    pub ctx: Context,
    series: FloatSeries,
}

impl HighestBars {
    pub fn new(ctx: Context, length: usize) -> Self {
        assert!(length >= 1, "HighestBars must have a length of at least 1");
        return Self {
            length,
            ctx: ctx.clone(),
            series: FloatSeries::new(ctx.clone()),
        };
    }
}

impl Incremental<f64, Option<i32>> for HighestBars {
    fn next(&mut self, value: f64) -> Option<i32> {
        self.series.next(value);

        if !self.series.is_filled(self.length) {
            return None;
        }

        return Some(highest_bars(&self.series.window(self.length), self.length));
    }
}
