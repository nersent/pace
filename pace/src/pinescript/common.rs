/// Same as `+` sum operator in PineScript.
pub fn ps_add(a: Option<f64>, b: Option<f64>) -> Option<f64> {
    match (a, b) {
        (Some(a), Some(b)) => Some(a + b),
        _ => None,
    }
}

/// Same as `-` difference operator in PineScript.
pub fn ps_diff(value: Option<f64>, prev_value: Option<f64>) -> Option<f64> {
    match (value, prev_value) {
        (Some(value), Some(prev_value)) => Some(value - prev_value),
        _ => None,
    }
}

/// Same as `/` division operator in PineScript.
pub fn ps_div(numerator: Option<f64>, denominator: Option<f64>) -> Option<f64> {
    match (numerator, denominator) {
        (Some(numerator), Some(denominator)) => {
            if denominator == 0.0 {
                return None;
            }
            Some(numerator / denominator)
        }
        _ => None,
    }
}

/// Same as `nz` in PineScript.
pub fn ps_nz(value: Option<f64>) -> f64 {
    value.unwrap_or(0.0)
}

/// Returns the maximum of two values. Same as `math.max` in PineScript.
pub fn ps_max(a: Option<f64>, b: Option<f64>) -> Option<f64> {
    match (a, b) {
        (Some(a), Some(b)) => Some(f64::max(a, b)),
        _ => None,
    }
}

/// Returns the minimum of two values. Same as `math.min` in PineScript.
pub fn ps_min(a: Option<f64>, b: Option<f64>) -> Option<f64> {
    match (a, b) {
        (Some(a), Some(b)) => Some(f64::min(a, b)),
        _ => None,
    }
}

/// Returns the absolute value of a number. Same as `math.abs` in PineScript.
pub fn ps_abs(value: Option<f64>) -> Option<f64> {
    return value.map(|v| v.abs());
}
