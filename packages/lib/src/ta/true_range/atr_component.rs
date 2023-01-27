use crate::{
    components::component_context::ComponentContext,
    ta::moving_average::rma_component::RunningMovingAverageComponent,
};

use super::true_range::TrueRange;

pub struct AverageTrueRangeComponent {
    pub length: usize,
    ctx: ComponentContext,
    rma: RunningMovingAverageComponent,
}

impl AverageTrueRangeComponent {
    pub fn new(ctx: ComponentContext, length: usize) -> Self {
        assert!(
            length > 0,
            "AverageTrueRangeComponent must have a length of at least 1"
        );
        return AverageTrueRangeComponent {
            ctx: ctx.clone(),
            length,
            rma: RunningMovingAverageComponent::new(ctx.clone(), length),
        };
    }

    pub fn next(&mut self) -> Option<f64> {
        let ctx = self.ctx.get();
        let (prev_high, prev_low, prev_close) = if ctx.current_tick == 0 {
            (None, None, None)
        } else {
            (ctx.prev_high(1), ctx.prev_low(1), ctx.prev_close(1))
        };
        let true_range = TrueRange::tr(
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
