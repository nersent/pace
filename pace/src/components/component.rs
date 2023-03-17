/// Recursive building block that accepts an input and produces an output imlicitly.
pub trait Component<T, R> {
    fn next(&mut self, input: T) -> R;
}
