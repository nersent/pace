use super::bars::{compute_highest, compute_lowest};

pub fn compute_stoch(
    value: Option<f64>,
    high: &[Option<f64>],
    low: &[Option<f64>],
    prev_stoch: Option<f64>,
) -> Option<f64> {
    value?;
    let high = compute_highest(high);
    let low = compute_lowest(low);

    if high.is_none() || low.is_none() {
        return None;
    }

    let diff = high.unwrap() - low.unwrap();

    if diff == 0.0 {
        return prev_stoch;
    }

    return Some(100.0 * (value.unwrap() - low.unwrap()) / diff);
}
