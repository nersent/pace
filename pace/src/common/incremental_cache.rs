use crate::core::{context::Context, incremental::Incremental};

/// Incremental Cache. Stores all values in a cache.
pub struct IncrementalCache<T> {
    pub ctx: Context,
    values: Vec<T>,
}

impl<T> IncrementalCache<T> {
    pub fn new(ctx: Context) -> Self {
        return Self {
            ctx: ctx.clone(),
            values: Vec::with_capacity(ctx.bars),
        };
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        let length = self.values.len();
        if index >= length {
            return None;
        }
        let index = (length - 1) - index;
        return self.values.get(index);
    }

    pub fn all(&self) -> &[T] {
        return &self.values;
    }

    pub fn last(&self) -> Option<&T> {
        return self.values.last();
    }

    pub fn first(&self) -> Option<&T> {
        return self.values.first();
    }

    pub fn size(&self) -> usize {
        return self.values.len();
    }

    /// Removes first value and returns it.
    pub fn shift(&mut self) -> T {
        return self.values.remove(0);
    }
}

impl<T> Incremental<T, ()> for IncrementalCache<T> {
    fn next(&mut self, value: T) {
        self.values.push(value);
    }
}
