use super::{component::Component, component_context::ComponentContext};

/// Stores alll values in a cache.
pub struct ValueCacheComponent<T> {
    pub ctx: ComponentContext,
    values: Vec<T>,
}

impl<T> ValueCacheComponent<T> {
    pub fn new(ctx: ComponentContext) -> Self {
        return Self {
            ctx: ctx.clone(),
            values: Vec::with_capacity(ctx.ticks()),
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

impl<T> Component<T, ()> for ValueCacheComponent<T> {
    fn next(&mut self, value: T) {
        self.values.push(value);
    }
}
