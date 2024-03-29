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
