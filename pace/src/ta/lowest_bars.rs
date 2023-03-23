use super::bars::lowest_bars;
use crate::core::{context::Context, incremental::Incremental};

/// Lowest value offset for a given number of bars back.
///
/// Same as PineScript `ta.lowestbars(src)`. Similar to `ta.lowestbars(src, length)`, but `length` is fixed and set on initialization.
pub struct LowestBars {
    pub length: usize,
    pub ctx: Context,
}

impl LowestBars {
    pub fn new(ctx: Context, length: usize) -> Self {
        assert!(length >= 1, "LowestBars must have a length of at least 1");
        return Self {
            length,
            ctx: ctx.clone(),
        };
    }
}

impl Incremental<(), Option<i32>> for LowestBars {
    fn next(&mut self, _: ()) -> Option<i32> {
        if !self.ctx.bar.at_length(self.length) {
            return None;
        }
        return lowest_bars(self.ctx.lows(self.length), self.length);
    }
}
