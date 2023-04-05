pub fn profit_factor(gross_profit: f64, gross_loss: f64) -> f64 {
    if gross_loss == 0.0 {
        return f64::NAN;
    }
    return gross_profit / gross_loss;
}

pub fn long_net_profit_ratio(long_net_profit: f64, short_net_profit: f64) -> f64 {
    if short_net_profit == 0.0 {
        return f64::NAN;
    }
    return long_net_profit / short_net_profit * -1.0;
}

pub fn percent_profitable(profitable_trades: usize, total_trades: usize) -> f64 {
    if total_trades == 0 {
        return f64::NAN;
    }
    return (profitable_trades as f64) / (total_trades as f64);
}

pub fn avg_trade(net_profit: f64, closed_trades: usize) -> f64 {
    if closed_trades == 0 {
        return f64::NAN;
    }
    return net_profit / (closed_trades as f64);
}

pub fn avg_winning_trade(gross_profit: f64, winning_trades: usize) -> f64 {
    if winning_trades == 0 {
        return f64::NAN;
    }
    return gross_profit / (winning_trades as f64);
}

pub fn avg_losing_trade(gross_loss: f64, losing_trades: usize) -> f64 {
    if losing_trades == 0 {
        return f64::NAN;
    }
    return gross_loss / (losing_trades as f64);
}

pub fn avg_win_loss_ratio(avg_winning_trade: f64, avg_losing_trade: f64) -> f64 {
    if avg_losing_trade == 0.0 {
        return f64::NAN;
    }
    return avg_winning_trade / avg_losing_trade;
}

pub fn omega_ratio(
    positive_returns_sum: f64,
    negative_returns_sum: f64,
    risk_free_rate: f64,
) -> f64 {
    if negative_returns_sum == 0.0 {
        return f64::NAN;
    }
    return (positive_returns_sum - risk_free_rate) / negative_returns_sum;
}

pub fn sharpe_ratio(mean_returns: f64, std_returns: f64, risk_free_rate: f64) -> f64 {
    if std_returns == 0.0 {
        return f64::NAN;
    }
    return (mean_returns - risk_free_rate) / std_returns;
}

pub fn sortino_ratio(mean_returns: f64, negative_returns_stdev: f64, risk_free_rate: f64) -> f64 {
    if negative_returns_stdev == 0.0 {
        return f64::NAN;
    }
    return (mean_returns - risk_free_rate) / negative_returns_stdev;
}

pub fn net_profit_percent(net_profit: f64, initial_capital: f64) -> f64 {
    return net_profit / initial_capital;
}

pub fn gross_profit_percent(gross_profit: f64, initial_capital: f64) -> f64 {
    return gross_profit / initial_capital;
}

pub fn gross_loss_percent(gross_loss: f64, initial_capital: f64) -> f64 {
    return gross_loss / initial_capital;
}

pub fn long_net_profit_percent(long_net_profit: f64, initial_capital: f64) -> f64 {
    return long_net_profit / initial_capital;
}

pub fn short_net_profit_percent(short_net_profit: f64, initial_capital: f64) -> f64 {
    return short_net_profit / initial_capital;
}

pub fn max_drawdown_percent(max_dd: f64, net_equity_max: f64) -> f64 {
    return max_dd / net_equity_max;
}

pub fn max_run_up_percent(max_run_up: f64, bar_equity_max: f64) -> f64 {
    return max_run_up / bar_equity_max;
}

pub fn returns(current_equity: f64, previous_equity: f64) -> f64 {
    if previous_equity == 0.0 || current_equity == 0.0 {
        return 0.0;
    }
    return (current_equity / previous_equity) - 1.0;
}
