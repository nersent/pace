use crate::core::{context::Context, incremental::Incremental};

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum CrossMode {
    Over,
    Under,
}

pub fn cross_over(
    current_a_value: f64,
    current_b_value: f64,
    previous_a_value: f64,
    previous_b_value: f64,
) -> bool {
    return (current_a_value > current_b_value) && (previous_a_value <= previous_b_value);
}

pub fn cross_under(
    current_a_value: f64,
    current_b_value: f64,
    previous_a_value: f64,
    previous_b_value: f64,
) -> bool {
    return (current_a_value < current_b_value) && (previous_a_value >= previous_b_value);
}

/// Same as PineScript `ta.cross(a, b)`.
pub struct Cross {
    pub ctx: Context,
    prev_a_value: f64,
    prev_b_value: f64,
}

impl Cross {
    pub fn new(ctx: Context) -> Self {
        return Cross {
            ctx,
            prev_a_value: f64::NAN,
            prev_b_value: f64::NAN,
        };
    }
}

impl Incremental<(f64, f64), Option<CrossMode>> for Cross {
    fn next(&mut self, (a, b): (f64, f64)) -> Option<CrossMode> {
        let mut mode = None;

        if !self.prev_a_value.is_nan() && !self.prev_b_value.is_nan() && !a.is_nan() && !b.is_nan()
        {
            if cross_over(a, b, self.prev_a_value, self.prev_b_value) {
                mode = Some(CrossMode::Over);
            } else if cross_under(a, b, self.prev_a_value, self.prev_b_value) {
                mode = Some(CrossMode::Under);
            }
        }

        self.prev_a_value = a;
        self.prev_b_value = b;

        return mode;
    }
}
