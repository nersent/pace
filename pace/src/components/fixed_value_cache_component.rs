use super::{component::Component, component_context::ComponentContext};

/// Stores N values in a cache.
pub struct FixedValueCacheComponent {
    pub ctx: ComponentContext,
    pub length: usize,
    values: Vec<Option<f64>>,
}

impl FixedValueCacheComponent {
    pub fn new(ctx: ComponentContext, length: usize) -> Self {
        return Self {
            ctx: ctx.clone(),
            values: Vec::with_capacity(ctx.ticks()),
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
}

impl Component<Option<f64>, ()> for FixedValueCacheComponent {
    fn next(&mut self, value: Option<f64>) {
        self.values.push(value);
    }
}
