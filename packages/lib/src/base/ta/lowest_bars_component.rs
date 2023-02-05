use crate::base::{
    components::component_context::ComponentContext,
    ta::{
        ema_component::ExponentialMovingAverageComponent,
        rma_component::RunningMovingAverageComponent, sma_component::SimpleMovingAverageComponent,
    },
};

use super::bars::{compute_highest_bars, compute_lowest_bars};

pub struct LowestBarsComponent {
    length: usize,
    ctx: ComponentContext,
}

impl LowestBarsComponent {
    pub fn new(ctx: ComponentContext, length: usize) -> Self {
        return LowestBarsComponent {
            length,
            ctx: ctx.clone(),
        };
    }

    pub fn next(&mut self) -> Option<i32> {
        let ctx = self.ctx.get();
        return compute_lowest_bars(ctx.prev_lows(self.length), self.length);
    }
}
