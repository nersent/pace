use super::bars::lowest_bars;
use crate::components::{component::Component, component_context::ComponentContext};

/// Lowest value offset for a given number of bars back.
pub struct LowestBarsComponent {
    pub length: usize,
    pub ctx: ComponentContext,
}

impl LowestBarsComponent {
    pub fn new(ctx: ComponentContext, length: usize) -> Self {
        return Self {
            length,
            ctx: ctx.clone(),
        };
    }
}

impl Component<(), Option<i32>> for LowestBarsComponent {
    fn next(&mut self, _: ()) -> Option<i32> {
        if !self.ctx.at_length(self.length) {
            return None;
        }
        return lowest_bars(self.ctx.prev_lows(self.length), self.length);
    }
}
