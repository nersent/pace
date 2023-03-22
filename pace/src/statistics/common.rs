pub fn mean(values: &[f64]) -> f64 {
    let n = values.len() as f64;
    let sum = values.iter().sum::<f64>();
    let mean = sum / n;
    return mean;
}

pub fn var_from_mean(values: &[f64], mean: f64) -> f64 {
    return values.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / values.len() as f64;
}

pub fn var(values: &[f64]) -> f64 {
    return var_from_mean(values, mean(values));
}

pub fn stdev(values: &[f64]) -> f64 {
    return stdev_from_var(var(values));
}

pub fn stdev_from_var(var: f64) -> f64 {
    return var.sqrt();
}
