use std::collections::VecDeque;

use crate::base::component_context::ComponentContext;

pub struct RecursiveLIFO {
    pub length: usize,
    ctx: ComponentContext,
    values: VecDeque<Option<f64>>,
    is_filled: bool,
    size: usize,
}

// Last in, first out
impl RecursiveLIFO {
    pub fn new(ctx: ComponentContext, length: usize) -> Self {
        assert!(length > 1, "RecursiveLIFO must have a size of at least 2");
        return RecursiveLIFO {
            ctx,
            length: length,
            size: 0,
            values: VecDeque::with_capacity(length),
            is_filled: false,
        };
    }

    pub fn next(&mut self, value: Option<f64>) -> (Option<f64>, Option<f64>, bool) {
        self.ctx.assert();

        let mut first_value: Option<f64> = None;
        self.values.push_front(value);
        self.size += 1;

        if self.size >= self.length {
            self.is_filled = true;

            first_value = self.values.pop_back().unwrap();
            self.size -= 1;
            //     if self.size > 1 {
            //         first_value = self.values.pop_back().unwrap();
            //         self.size -= 1;
            //     }
        }

        return (first_value, value, self.is_filled);
    }
}
