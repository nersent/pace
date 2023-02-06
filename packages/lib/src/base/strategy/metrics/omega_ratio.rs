pub fn compute_omega_ratio(positive_returns_sum: f64, negative_returns_sum: f64) -> f64 {
    if negative_returns_sum == 0.0 {
        return 0.0;
    }
    return positive_returns_sum / negative_returns_sum;
}
