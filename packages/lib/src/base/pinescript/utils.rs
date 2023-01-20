pub fn ps_nz(value: Option<f64>) -> f64 {
    match value {
        Some(v) => v,
        None => 0.0,
    }
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
