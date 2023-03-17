use super::{component::Component, component_context::ComponentContext};

/// Stores **`N`** last values in a cache.
pub struct WindowCacheComponent<T> {
    pub ctx: ComponentContext,
    pub length: usize,
    values: Vec<T>,
}

impl<T> WindowCacheComponent<T> {
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

    /// Returns **`N - I`** previous value.
    pub fn get(&mut self, index: usize) -> Option<&T> {
        let index = (self.values.len() - 1) - index;
        return self.values.get(index);
    }

    /// Returns all **`N`** previous values.
    pub fn all(&mut self) -> &[T] {
        let size = self.values.len();
        let start_index = if size < self.length {
            0
        } else {
            size - (self.length)
        };
        return &self.values[start_index..];
    }

    /// Returns previous value.
    pub fn last(&mut self) -> Option<&T> {
        return self.values.last();
    }

    /// Returns **`N`** previous value (first value of the window).
    pub fn first(&mut self) -> Option<&T> {
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

impl WindowCacheComponent<Option<f64>> {
    fn normalize_value(value: Option<&Option<f64>>) -> Option<f64> {
        if value.is_none() {
            return None;
        }
        return *value.unwrap();
    }

    pub fn get_unwrapped(&mut self, index: usize) -> Option<f64> {
        return Self::normalize_value(self.get(index));
    }

    pub fn last_unwrapped(&mut self) -> Option<f64> {
        return Self::normalize_value(self.last());
    }

    pub fn first_unwrapped(&mut self) -> Option<f64> {
        return Self::normalize_value(self.first());
    }
}

impl<T> Component<T, ()> for WindowCacheComponent<T> {
    fn next(&mut self, value: T) {
        self.values.push(value);
    }
}
