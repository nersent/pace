use std::collections::VecDeque;

use crate::components::component_context::ComponentContext;

pub struct RecursiveLIFO {
    ctx: ComponentContext,
    size: usize,
    values: VecDeque<Option<f64>>,
    pub is_filled: bool,
}

// Last in, first out
impl RecursiveLIFO {
    pub fn new(ctx: ComponentContext, size: usize) -> Self {
        assert!(size > 0, "RecursiveLIFO must have a size of at least 1");
        return RecursiveLIFO {
            ctx,
            size,
            values: VecDeque::new(),
            is_filled: false,
        };
    }

    pub fn next(&mut self, value: Option<f64>) -> (Option<f64>, Option<f64>, bool) {
        self.ctx.assert();
        let mut first_value: Option<f64> = None;
        self.values.push_front(value);

        let len = self.values.len();

        if len >= self.size {
            self.is_filled = true;

            if len > 1 {
                first_value = self.values.pop_back().unwrap();
            }
        }

        return (first_value, value, self.is_filled);
    }
}
