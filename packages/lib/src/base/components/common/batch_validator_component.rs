use crate::base::components::component_context::ComponentContext;

use super::position_component::PositionComponent;

pub struct BatchValidatorComponent {
    pub length: usize,
    ctx: ComponentContext,
    last_none_index: usize,
    was_none: bool,
    position: PositionComponent,
}

// Checks if items within a batch of size "length" are valid.
impl BatchValidatorComponent {
    pub fn new(ctx: ComponentContext, length: usize) -> Self {
        assert!(
            length > 0,
            "BatchValidatorComponent must have a length of at least 1"
        );
        return BatchValidatorComponent {
            ctx: ctx.clone(),
            length,
            last_none_index: 0,
            was_none: false,
            position: PositionComponent::new(ctx.clone()),
        };
    }

    pub fn next(&mut self, value: Option<f64>) -> bool {
        self.ctx.assert();

        let current_index = self.position.next();

        if value.is_none() {
            self.last_none_index = current_index;
            self.was_none = true;
            return false;
        }

        return !self.was_none || (current_index - self.last_none_index >= self.length);
    }
}
