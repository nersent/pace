use crate::components::component_context::ComponentContext;

pub struct RecursivePosition {
    ctx: ComponentContext,
    index: usize,
}

impl RecursivePosition {
    pub fn new(ctx: ComponentContext) -> Self {
        return RecursivePosition {
            ctx: ctx.clone(),
            index: 0,
        };
    }

    pub fn next(&mut self) -> usize {
        self.ctx.assert();
        let prev_index = self.index;
        self.index += 1;
        return prev_index;
    }
}
