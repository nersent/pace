use crate::core::{context::Context, incremental::Incremental};

use super::cross::{cross_over, cross_under, CrossMode};

/// Same as PineScript `ta.crossunder(a, b)`.
pub struct CrossUnder {
    pub ctx: Context,
    prev_a_value: Option<f64>,
    prev_b_value: Option<f64>,
}

impl CrossUnder {
    pub fn new(ctx: Context) -> Self {
        return Self {
            ctx,
            prev_a_value: None,
            prev_b_value: None,
        };
    }
}

impl Incremental<(Option<f64>, Option<f64>), bool> for CrossUnder {
    fn next(&mut self, (a, b): (Option<f64>, Option<f64>)) -> bool {
        let cross = match (self.prev_a_value, self.prev_b_value, a, b) {
            (Some(prev_a), Some(prev_b), Some(a), Some(b)) => cross_under(a, b, prev_a, prev_b),
            _ => false,
        };

        self.prev_a_value = a;
        self.prev_b_value = b;

        return cross;
    }
}
