use crate::components::{component::Component, component_context::ComponentContext};

use super::bars::highest_bars;

/// Highest value offset for a given number of bars back.
pub struct HighestBarsComponent {
    pub length: usize,
    pub ctx: ComponentContext,
}

impl HighestBarsComponent {
    pub fn new(ctx: ComponentContext, length: usize) -> Self {
        return Self {
            length,
            ctx: ctx.clone(),
        };
    }
}

impl Component<(), Option<i32>> for HighestBarsComponent {
    fn next(&mut self, _: ()) -> Option<i32> {
        if !self.ctx.at_length(self.length) {
            return None;
        }
        return highest_bars(self.ctx.prev_highs(self.length), self.length);
    }
}
