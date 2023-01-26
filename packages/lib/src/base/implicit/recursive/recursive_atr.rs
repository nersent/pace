use crate::base::{
    component_context::ComponentContext, explicit::explicit_true_range::compute_true_range,
    implicit::recursive::recursive_rma::RecursiveRMA,
};

pub struct RecursiveATR {
    pub length: usize,
    ctx: ComponentContext,
    rma: RecursiveRMA,
}

impl RecursiveATR {
    pub fn new(ctx: ComponentContext, length: usize) -> Self {
        assert!(length > 0, "RecursiveATR must have a length of at least 1");
        return RecursiveATR {
            ctx: ctx.clone(),
            length,
            rma: RecursiveRMA::new(ctx.clone(), length),
        };
    }

    pub fn next(&mut self) -> Option<f64> {
        let ctx = self.ctx.get();
        let (prev_high, prev_low, prev_close) = if ctx.current_tick == 0 {
            (None, None, None)
        } else {
            (ctx.prev_high(1), ctx.prev_low(1), ctx.prev_close(1))
        };
        let true_range = compute_true_range(
            ctx.high().unwrap(),
            ctx.low().unwrap(),
            prev_high,
            prev_low,
            prev_close,
            true,
        );
        let atr = self.rma.next(true_range);
        return atr;
    }
}
