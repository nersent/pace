pub fn compute_profit_factor(gross_profit: f64, gross_loss: f64) -> f64 {
    if gross_loss == 0.0 {
        return 0.0;
    }
    return gross_profit / gross_loss;
}
