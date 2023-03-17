#[derive(PartialEq, Copy, Clone, Debug)]
pub enum CrossMode {
    Over,
    Under,
}

pub fn compute_cross_over(
    current_a_value: f64,
    current_b_value: f64,
    previous_a_value: f64,
    previous_b_value: f64,
) -> bool {
    return (current_a_value > current_b_value) && (previous_a_value <= previous_b_value);
}

pub fn compute_cross_under(
    current_a_value: f64,
    current_b_value: f64,
    previous_a_value: f64,
    previous_b_value: f64,
) -> bool {
    return (current_a_value < current_b_value) && (previous_a_value >= previous_b_value);
}
