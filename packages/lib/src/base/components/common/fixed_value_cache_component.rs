use std::collections::VecDeque;

use crate::base::components::component_context::ComponentContext;

pub struct FixedValueCacheComponent {
    pub length: usize,
    ctx: ComponentContext,
    values: Vec<Option<f64>>,
}

impl FixedValueCacheComponent {
    pub fn new(ctx: ComponentContext, length: usize) -> Self {
        return FixedValueCacheComponent {
            ctx,
            values: Vec::new(),
            length,
        };
    }

    pub fn is_filled(&self) -> bool {
        return self.values.len() >= self.length;
    }

    pub fn get(&mut self, index: usize) -> Option<f64> {
        let index = (self.values.len() - 1) - index;
        return *self.values.get(index).unwrap();
    }

    pub fn all(&mut self) -> &[Option<f64>] {
        let size = self.values.len();
        let start_index = if size < self.length {
            0
        } else {
            size - (self.length)
        };
        return &self.values[start_index..];
    }

    pub fn last(&mut self) -> Option<f64> {
        return *self.values.last().unwrap();
    }

    pub fn first(&mut self) -> Option<f64> {
        let size = self.values.len();
        if size < self.length {
            return None;
        }
        return self.get(self.length - 1);
    }

    pub fn size(&self) -> usize {
        return self.values.len();
    }

    pub fn next(&mut self, value: Option<f64>) {
        self.ctx.on_next();
        self.values.push(value);
    }
}
