use crate::core::{context::Context, incremental::Incremental};
use std::fmt::Debug;

use super::array_snapshot::{ArraySnapshot, Compare};

impl Compare<Option<f64>> for Option<f64> {
    fn compare(&self, other: &Option<f64>) -> bool {
        match (self, other) {
            (Some(a), Some(b)) => (a - b).abs() < 0.0001,
            (None, None) => true,
            _ => false,
        }
    }
}

pub fn test_incremental<R: Compare<R> + Debug>(
    ctx: Context,
    mut target: Box<dyn Incremental<(), R>>,
    expected: &[R],
) {
    let mut snapshot = ArraySnapshot::<R>::new();
    for _ in ctx.clone() {
        let output = target.next(());
        snapshot.push(output);
    }

    snapshot.assert_iter(&expected, |actual, expected| {
        return actual.compare(expected);
    });
}
