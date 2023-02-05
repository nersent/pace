use crate::base::components::component_context::ComponentContext;

pub struct PositionComponent {
    ctx: ComponentContext,
    index: usize,
}

impl PositionComponent {
    pub fn new(ctx: ComponentContext) -> Self {
        return PositionComponent { ctx, index: 0 };
    }

    pub fn next(&mut self) -> usize {
        self.ctx.assert();
        let prev_index = self.index;
        self.index += 1;
        return prev_index;
    }
}
