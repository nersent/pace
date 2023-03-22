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

    pub fn get(&mut self, index: usize) -> &T {
        let index = (self.values.len() - 1) - index;
        return self.values.get(index).unwrap();
    }

    pub fn all(&mut self) -> &[T] {
        return &self.values;
    }

    pub fn last(&mut self) -> &T {
        return self.values.last().unwrap();
    }

    pub fn first(&mut self) -> &T {
        return self.values.first().unwrap();
    }

    pub fn size(&self) -> usize {
        return self.values.len();
    }
}

impl<T> Incremental<T, ()> for IncrementalCache<T> {
    fn next(&mut self, value: T) {
        self.values.push(value);
    }
}
