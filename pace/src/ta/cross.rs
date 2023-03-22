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
    prev_a_value: Option<f64>,
    prev_b_value: Option<f64>,
}

impl Cross {
    pub fn new(ctx: Context) -> Self {
        return Cross {
            ctx,
            prev_a_value: None,
            prev_b_value: None,
        };
    }
}

impl Incremental<(Option<f64>, Option<f64>), Option<CrossMode>> for Cross {
    fn next(&mut self, (a, b): (Option<f64>, Option<f64>)) -> Option<CrossMode> {
        let cross = match (self.prev_a_value, self.prev_b_value, a, b) {
            (Some(prev_a), Some(prev_b), Some(a), Some(b)) => {
                if cross_over(a, b, prev_a, prev_b) {
                    Some(CrossMode::Over)
                } else if cross_under(a, b, prev_a, prev_b) {
                    Some(CrossMode::Under)
                } else {
                    None
                }
            }
            _ => None,
        };

        self.prev_a_value = a;
        self.prev_b_value = b;

        return cross;
    }
}
