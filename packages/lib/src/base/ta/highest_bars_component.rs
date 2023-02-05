use crate::base::{
    components::component_context::ComponentContext,
    ta::{
        ema_component::ExponentialMovingAverageComponent,
        rma_component::RunningMovingAverageComponent, sma_component::SimpleMovingAverageComponent,
    },
};

use super::bars::compute_highest_bars;

pub struct HighestBarsComponent {
    length: usize,
    ctx: ComponentContext,
}

impl HighestBarsComponent {
    pub fn new(ctx: ComponentContext, length: usize) -> Self {
        return HighestBarsComponent {
            length,
            ctx: ctx.clone(),
        };
    }

    pub fn next(&mut self) -> Option<i32> {
        let ctx = self.ctx.get();
        return compute_highest_bars(ctx.prev_highs(self.length), self.length);
    }
}
