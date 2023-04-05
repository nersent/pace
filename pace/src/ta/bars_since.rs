use crate::core::{context::Context, incremental::Incremental};

/// Same as PineScript `ta.barssince(condition)`
pub struct BarsSince {
    pub ctx: Context,
    bars: Option<usize>,
}

impl BarsSince {
    pub fn new(ctx: Context) -> Self {
        return Self {
            ctx: ctx.clone(),
            bars: None,
        };
    }
}

impl Incremental<bool, Option<usize>> for BarsSince {
    fn next(&mut self, condition: bool) -> Option<usize> {
        if condition {
            self.bars = Some(0);
        } else if let Some(bars) = self.bars {
            self.bars = Some(bars + 1);
        }

        return self.bars;
    }
}
