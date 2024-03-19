use crate::core::{context::Context, incremental::Incremental};

pub struct FloatSeries {
    pub ctx: Context,
    pub values: Vec<f64>,
    current_offset: usize,
}

impl FloatSeries {
    pub fn new(ctx: Context) -> Self {
        return Self {
            ctx: ctx.clone(),
            values: Vec::with_capacity(ctx.bars),
            current_offset: 0,
        };
    }

    pub fn with_initial_value(mut self, value: f64) -> Self {
        self.values.push(value);
        return self;
    }

    fn push(&mut self, value: f64) {
        self.current_offset = self.values.len();
        self.values.push(value);
    }

    pub fn size(&self) -> usize {
        return self.values.len();
    }

    pub fn is_filled(&self, size: usize) -> bool {
        return self.values.len() >= size;
    }

    /// Returns **`N - I`** previous value.
    pub fn get(&self, index: usize) -> f64 {
        if index >= self.values.len() {
            return f64::NAN;
        }
        let index = (self.values.len() - 1) - index;
        return self.values[index];
    }

    pub fn at(&self, index: usize) -> f64 {
        if index >= self.values.len() {
            return f64::NAN;
        }
        return self.values[index];
    }

    // /// Returns all **`N`** previous values.
    pub fn window(&self, length: usize) -> &[f64] {
        return &self.values[self.values.len() - length..];
    }

    // pub fn offset_window(&self, length: usize, offset: usize) -> &[f64] {
    //     let size = self.values.len();
    //     return &self.values[size - length - offset..size - offset];
    // }

    // /// Returns previous value.
    // pub fn last(&mut self) -> Option<&T> {
    //     return self.values.last();
    // }

    // /// Returns **`N`** previous value (first value of the window).
    // pub fn first(&mut self) -> Option<&T> {
    //     let size = self.values.len();
    //     if size < self.length {
    //         return None;
    //     }
    //     return self.get(self.length - 1);
    // }
}

impl std::ops::Index<usize> for FloatSeries {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        return &self.values[self.current_offset - index];
    }
}

impl Incremental<f64, ()> for FloatSeries {
    fn next(&mut self, value: f64) {
        self.push(value);
    }
}
