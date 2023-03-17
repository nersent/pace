use super::{component::Component, component_context::ComponentContext};

/// Returns `None` until gets **`N`** `non-None` items in a row. This allows to keep the same behaviour as in PineScript.
pub struct BatchValidatorComponent {
    pub ctx: ComponentContext,
    pub length: usize,
    last_none_index: usize,
    was_none: bool,
}

impl BatchValidatorComponent {
    pub fn new(ctx: ComponentContext, length: usize) -> Self {
        assert!(
            length >= 1,
            "BatchValidatorComponent must have a length of at least 1"
        );
        return Self {
            ctx: ctx.clone(),
            length,
            last_none_index: 0,
            was_none: false,
        };
    }
}

impl Component<Option<f64>, bool> for BatchValidatorComponent {
    fn next(&mut self, value: Option<f64>) -> bool {
        let current_index = self.ctx.bar_index();

        if value.is_none() {
            self.last_none_index = current_index;
            self.was_none = true;
            return false;
        }

        return !self.was_none || (current_index - self.last_none_index >= self.length);
    }
}