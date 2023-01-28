pub fn compute_pnl(fill_size: f64, fill_price: f64, current_price: f64, is_long: bool) -> f64 {
    let multiplier = if is_long { 1.0 } else { -1.0 };
    return (current_price - fill_price) * fill_size * multiplier;
}

pub fn compute_fill_size(equity: f64, current_price: f64) -> f64 {
    return equity / current_price;
}

pub fn compute_return(current_equity: f64, previous_equity: f64) -> f64 {
    return current_equity / previous_equity - 1.0;
}

pub fn compute_sharpe_ratio(mean_returns: f64, std_returns: f64, risk_free_rate: f64) -> f64 {
    if (std_returns == 0.0) {
        return 0.0;
    }
    return (mean_returns - risk_free_rate) / std_returns;
}

pub fn compute_omega_ratio(positive_returns_sum: f64, negative_returns_sum: f64) -> f64 {
    if (negative_returns_sum == 0.0) {
        return 0.0;
    }
    return positive_returns_sum / negative_returns_sum;
}
