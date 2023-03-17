pub fn compute_profit_factor(gross_profit: f64, gross_loss: f64) -> Option<f64> {
    if gross_loss == 0.0 {
        return None;
    }
    return Some(gross_profit / gross_loss);
}

pub fn compute_long_net_profit_ratio(long_net_profit: f64, short_net_profit: f64) -> Option<f64> {
    if short_net_profit == 0.0 {
        return None;
    }
    return Some(long_net_profit / short_net_profit * -1.0);
}

pub fn compute_percent_profitable_trades(
    profitable_trades: usize,
    total_trades: usize,
) -> Option<f64> {
    if total_trades == 0 {
        return None;
    }
    return Some((profitable_trades as f64) / (total_trades as f64));
}

pub fn compute_avg_trade(net_profit: f64, closed_trades: usize) -> Option<f64> {
    if closed_trades == 0 {
        return None;
    }
    return Some(net_profit / (closed_trades as f64));
}

pub fn compute_avg_winning_trade(gross_profit: f64, winning_trades: usize) -> Option<f64> {
    if winning_trades == 0 {
        return None;
    }
    return Some(gross_profit / (winning_trades as f64));
}

pub fn compute_avg_losing_trade(gross_loss: f64, losing_trades: usize) -> Option<f64> {
    if losing_trades == 0 {
        return None;
    }
    return Some(gross_loss / (losing_trades as f64));
}

pub fn compute_avg_win_loss_ratio(avg_winning_trade: f64, avg_losing_trade: f64) -> Option<f64> {
    if avg_losing_trade == 0.0 {
        return None;
    }
    return Some(avg_winning_trade / avg_losing_trade);
}
