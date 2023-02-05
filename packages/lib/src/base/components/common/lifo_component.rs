use std::collections::VecDeque;

use crate::base::components::component_context::ComponentContext;

pub struct LifoComponent {
    ctx: ComponentContext,
    size: usize,
    values: Vec<Option<f64>>,
    // values: VecDeque<Option<f64>>,
    pub is_filled: bool,
}

// Last in, first out
impl LifoComponent {
    pub fn new(ctx: ComponentContext, size: usize) -> Self {
        assert!(size > 0, "LifoComponent must have a size of at least 1");
        return LifoComponent {
            ctx,
            size,
            // values: VecDeque::new(),
            values: Vec::new(),
            is_filled: false,
        };
    }

    pub fn get(&mut self, index: usize) -> &Option<f64> {
        let index = (self.values.len() - 1) - index;
        return self.values.get(index).unwrap();
    }

    pub fn values(&mut self) -> &[Option<f64>] {
        let start_index = if self.values.len() < self.size {
            0
        } else {
            self.values.len() - (self.size - 1)
        };
        return &self.values[start_index..];
        // self.values.make_contiguous();
        // return self.values.as_slices().0;
    }

    pub fn values_with_first(&mut self) -> &[Option<f64>] {
        let start_index = self.values.len() - self.size;
        return &self.values[start_index..];
    }

    pub fn next(&mut self, value: Option<f64>) -> (Option<f64>, Option<f64>, bool) {
        self.ctx.assert();
        let mut first_value: Option<f64> = None;
        // self.values.push_front(value);
        self.values.push(value);

        let len = self.values.len();

        if len >= self.size {
            self.is_filled = true;

            if len > 1 {
                first_value = *self.get(self.size - 1);
                // let index = len - (self.size - 1);
                // first_value = self.values.get(index).unwrap().clone();
                // first_value = self.values.pop_back().unwrap();
            }
        }

        return (first_value, value, self.is_filled);
    }
}
