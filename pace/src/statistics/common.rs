pub fn variance(values: &[f64], mean: f64) -> f64 {
    let n = values.len() as f64;
    let variance = values.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / n;
    return variance;
}

pub fn stdev_from_var(var: f64) -> f64 {
    return var.sqrt();
}
