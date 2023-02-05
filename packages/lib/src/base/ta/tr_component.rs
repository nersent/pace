use crate::base::components::component_context::ComponentContext;

use super::tr::compute_true_range;

pub struct TrueRangeComponent {
    pub handle_na: bool,
    ctx: ComponentContext,
}

impl TrueRangeComponent {
    pub fn new(ctx: ComponentContext, handle_na: bool) -> Self {
        return TrueRangeComponent {
            ctx: ctx.clone(),
            handle_na,
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
            self.handle_na,
        );
        return true_range;
    }
}
