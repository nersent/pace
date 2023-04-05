use crate::core::{context::Context, incremental::Incremental};

use super::cross::cross_over;

/// Same as PineScript `ta.crossover(a, b)`.
pub struct CrossOver {
    pub ctx: Context,
    prev_a_value: f64,
    prev_b_value: f64,
}

impl CrossOver {
    pub fn new(ctx: Context) -> Self {
        return Self {
            ctx,
            prev_a_value: f64::NAN,
            prev_b_value: f64::NAN,
        };
    }
}

impl Incremental<(f64, f64), bool> for CrossOver {
    fn next(&mut self, (a, b): (f64, f64)) -> bool {
        let cross = !self.prev_a_value.is_nan()
            && !self.prev_b_value.is_nan()
            && !a.is_nan()
            && !b.is_nan()
            && cross_over(a, b, self.prev_a_value, self.prev_b_value);

        self.prev_a_value = a;
        self.prev_b_value = b;

        return cross;
    }
}
