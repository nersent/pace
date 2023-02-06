pub fn compute_sharpe_ratio(mean_returns: f64, std_returns: f64, risk_free_rate: f64) -> f64 {
    if std_returns == 0.0 {
        return 0.0;
    }
    return (mean_returns - risk_free_rate) / std_returns;
}
