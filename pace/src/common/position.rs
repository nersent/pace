use crate::core::{context::Context, incremental::Incremental};

pub struct Position {
    pub ctx: Context,
    index: usize,
}

impl Position {
    pub fn new(ctx: Context) -> Self {
        return Self { ctx, index: 0 };
    }
}

impl Incremental<(), usize> for Position {
    fn next(&mut self, _: ()) -> usize {
        let prev_index = self.index;
        self.index += 1;
        return prev_index;
    }
}
