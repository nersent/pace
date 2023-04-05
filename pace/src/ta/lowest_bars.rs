use super::bars::lowest_bars;
use crate::{
    common::float_series::FloatSeries,
    core::{context::Context, incremental::Incremental},
};

/// Lowest value offset for a given number of bars back.
///
/// Same as PineScript `ta.lowestbars(src)`. Similar to `ta.lowestbars(src, length)`, but `length` is fixed and set on initialization.
pub struct LowestBars {
    pub length: usize,
    pub ctx: Context,
    series: FloatSeries,
}

impl LowestBars {
    pub fn new(ctx: Context, length: usize) -> Self {
        assert!(length >= 1, "LowestBars must have a length of at least 1");
        return Self {
            length,
            ctx: ctx.clone(),
            series: FloatSeries::new(ctx.clone()),
        };
    }
}

impl Incremental<f64, Option<i32>> for LowestBars {
    fn next(&mut self, value: f64) -> Option<i32> {
        self.series.next(value);

        if !self.series.is_filled(self.length) {
            return None;
        }

        return Some(lowest_bars(&self.series.window(self.length), self.length));
    }
}
