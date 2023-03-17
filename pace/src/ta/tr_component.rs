use crate::components::{component::Component, component_context::ComponentContext};

use super::tr::true_range;

/// True range.
pub struct TrComponent {
    pub handle_na: bool,
    pub ctx: ComponentContext,
}

impl TrComponent {
    pub fn new(ctx: ComponentContext, handle_na: bool) -> Self {
        return Self {
            ctx: ctx.clone(),
            handle_na,
        };
    }
}

impl Component<(), Option<f64>> for TrComponent {
    fn next(&mut self, _: ()) -> Option<f64> {
        return true_range(
            self.ctx.high().unwrap(),
            self.ctx.low().unwrap(),
            self.ctx.prev_high(1),
            self.ctx.prev_low(1),
            self.ctx.prev_close(1),
            self.handle_na,
        );
    }
}
