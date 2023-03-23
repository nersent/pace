use crate::core::{context::Context, incremental::Incremental};

/// Returns `None` until gets **`N`** `non-None` items in a row. This allows to keep the same behaviour as in PineScript.
pub struct WindowValidator {
    pub ctx: Context,
    pub length: usize,
    last_none_index: usize,
    was_none: bool,
}

impl WindowValidator {
    pub fn new(ctx: Context, length: usize) -> Self {
        assert!(
            length >= 1,
            "WindowValidator must have a length of at least 1"
        );
        return Self {
            ctx: ctx.clone(),
            length,
            last_none_index: 0,
            was_none: false,
        };
    }
}

impl Incremental<Option<f64>, bool> for WindowValidator {
    fn next(&mut self, value: Option<f64>) -> bool {
        let current_index = self.ctx.bar.index();

        if value.is_none() {
            self.last_none_index = current_index;
            self.was_none = true;
            return false;
        }

        return !self.was_none || (current_index - self.last_none_index >= self.length);
    }
}
