use super::bars::{highest, lowest};

/// Stochastic.
///
/// Similar to PineScript `ta.stoch(src, high, low, length)`, but `src` array requires to be truncated to the length and you need to keep track of the previous value of stoch.
pub fn stoch(
    value: Option<f64>,
    high: &[Option<f64>],
    low: &[Option<f64>],
    prev_stoch: Option<f64>,
) -> Option<f64> {
    value?;
    let high = highest(high);
    let low = lowest(low);

    if high.is_none() || low.is_none() {
        return None;
    }

    let diff = high.unwrap() - low.unwrap();

    if diff == 0.0 {
        return prev_stoch;
    }

    return Some(100.0 * (value.unwrap() - low.unwrap()) / diff);
}
