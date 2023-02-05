pub fn ps_nz(value: Option<f64>) -> f64 {
    value.unwrap_or(0.0)
}

pub fn ps_max(a: Option<f64>, b: Option<f64>) -> Option<f64> {
    match (a, b) {
        (Some(a), Some(b)) => Some(f64::max(a, b)),
        _ => None,
    }
}

pub fn ps_min(a: Option<f64>, b: Option<f64>) -> Option<f64> {
    match (a, b) {
        (Some(a), Some(b)) => Some(f64::min(a, b)),
        _ => None,
    }
}

pub fn ps_diff(value: Option<f64>, prev_value: Option<f64>) -> Option<f64> {
    match (value, prev_value) {
        (Some(value), Some(prev_value)) => Some(value - prev_value),
        _ => None,
    }
}

pub fn ps_abs(value: Option<f64>) -> Option<f64> {
    return value.map(|v| v.abs());
}

pub fn is_equal(a: Option<f64>, b: Option<f64>) -> bool {
    match (a, b) {
        (Some(a), Some(b)) => a == b,
        (None, None) => true,
        _ => false,
    }
}
