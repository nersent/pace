use crate::core::{context::Context, incremental::Incremental};

use super::bars::highest_bars;

/// Highest value offset for a given number of bars back.
///
/// Same as PineScript `ta.highestbars(src)`. Similar to `ta.highestbars(src, length)`, but `length` is fixed and set on initialization.
pub struct HighestBars {
    pub length: usize,
    pub ctx: Context,
}

impl HighestBars {
    pub fn new(ctx: Context, length: usize) -> Self {
        assert!(length >= 1, "HighestBars must have a length of at least 1");
        return Self {
            length,
            ctx: ctx.clone(),
        };
    }
}

impl Incremental<(), Option<i32>> for HighestBars {
    fn next(&mut self, _: ()) -> Option<i32> {
        if !self.ctx.bar.at_length(self.length) {
            return None;
        }
        return highest_bars(self.ctx.highs(self.length), self.length);
    }
}
