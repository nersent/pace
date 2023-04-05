use crate::core::{context::Context, incremental::Incremental};

use super::cross::{cross_over, cross_under, CrossMode};

/// Same as PineScript `ta.crossunder(a, b)`.
pub struct CrossUnder {
    pub ctx: Context,
    prev_a_value: f64,
    prev_b_value: f64,
}

impl CrossUnder {
    pub fn new(ctx: Context) -> Self {
        return Self {
            ctx,
            prev_a_value: f64::NAN,
            prev_b_value: f64::NAN,
        };
    }
}

impl Incremental<(f64, f64), bool> for CrossUnder {
    fn next(&mut self, (a, b): (f64, f64)) -> bool {
        let cross = !self.prev_a_value.is_nan()
            && !self.prev_b_value.is_nan()
            && !a.is_nan()
            && !b.is_nan()
            && cross_under(a, b, self.prev_a_value, self.prev_b_value);

        self.prev_a_value = a;
        self.prev_b_value = b;

        return cross;
    }
}
