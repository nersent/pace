use super::{component::Component, component_context::ComponentContext};

pub struct PositionComponent {
    pub ctx: ComponentContext,
    index: usize,
}

impl PositionComponent {
    pub fn new(ctx: ComponentContext) -> Self {
        return Self { ctx, index: 0 };
    }
}

impl Component<(), usize> for PositionComponent {
    fn next(&mut self, _: ()) -> usize {
        let prev_index = self.index;
        self.index += 1;
        return prev_index;
    }
}
