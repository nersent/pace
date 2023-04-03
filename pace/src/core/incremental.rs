use crate::common::src::Src;

use super::context::Context;

/// Recursive building block that accepts an input and produces an output imlicitly.
pub trait Incremental<T, R> {
    /// It is recommended that `next` method is called on every tick, even if the input is `None`.
    fn next(&mut self, input: T) -> R;

    fn to_box(self) -> Box<Self>
    where
        Self: Sized,
    {
        return Box::new(self);
    }
}

/// A substitute for `Default` trait from Rust `std`, but with a context as an argument.
pub trait IncrementalDefault {
    fn default(ctx: Context) -> Self;
}

pub struct Chained<T, R, NR> {
    pub ctx: Context,
    pub current: Box<dyn Incremental<T, R>>,
    pub next: Box<dyn Incremental<R, NR>>,
}

impl<T, R, NR> Chained<T, R, NR> {
    pub fn new(
        ctx: Context,
        current: Box<dyn Incremental<T, R>>,
        next: Box<dyn Incremental<R, NR>>,
    ) -> Self {
        return Self { ctx, current, next };
    }
}

impl<T, R, NR> Incremental<T, NR> for Chained<T, R, NR> {
    fn next(&mut self, input: T) -> NR {
        let result = self.current.next(input);
        return self.next.next(result);
    }
}

pub struct Dummy {}

impl Incremental<(), ()> for Dummy {
    fn next(&mut self, _: ()) -> () {}
}
