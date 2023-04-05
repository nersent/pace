use crate::core::{context::Context, incremental::Incremental};

use super::{running_moving_average::Rma, true_range::Tr};

/// Average True Range.
///
/// Same as PineScript `ta.atr(src)`. Similar to `ta.atr(src, length)`, but `length` is fixed and set on initialization.
pub struct Atr {
    pub length: usize,
    pub ctx: Context,
    tr: Tr,
    rma: Rma,
}

impl Atr {
    pub fn new(ctx: Context, length: usize) -> Self {
        assert!(length >= 1, "Atr must have a length of at least 1");
        return Self {
            ctx: ctx.clone(),
            length,
            tr: Tr::new(ctx.clone(), true),
            rma: Rma::new(ctx.clone(), length),
        };
    }
}

impl Incremental<(), f64> for Atr {
    fn next(&mut self, _: ()) -> f64 {
        let true_range = self.tr.next(());
        let atr = self.rma.next(true_range);
        return atr;
    }
}
