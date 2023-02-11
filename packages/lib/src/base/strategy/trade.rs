use colored::{ColoredString, Colorize};
use pyo3::pyclass;

use crate::base::ta::cross::CrossMode;

#[derive(Debug, PartialEq, Clone, Copy)]
#[pyclass(name = "TradeDirection")]
pub enum TradeDirection {
    Long = 0,
    Short = 1,
}

impl TradeDirection {
    pub fn get_opposite(&self) -> Self {
        return match self {
            TradeDirection::Long => TradeDirection::Short,
            TradeDirection::Short => TradeDirection::Long,
        };
    }
}

pub fn trade_direction_to_f64(direction: Option<TradeDirection>) -> f64 {
    return match direction {
        Some(TradeDirection::Long) => 1.0,
        Some(TradeDirection::Short) => -1.0,
        None => 0.0,
    };
}

pub fn trade_direction_from_f64(value: Option<f64>) -> Option<TradeDirection> {
    return match value {
        Some(value) => {
            if value == 1.0 {
                return Some(TradeDirection::Long);
            }
            if value == -1.0 {
                return Some(TradeDirection::Short);
            }
            return None;
        }
        None => None,
    };
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[pyclass(name = "Trade")]
pub struct Trade {
    #[pyo3(get)]
    pub direction: TradeDirection,
    #[pyo3(get)]
    pub is_closed: bool,
    #[pyo3(get)]
    pub entry_tick: Option<usize>,
    #[pyo3(get)]
    pub entry_price: Option<f64>,
    #[pyo3(get)]
    pub exit_tick: Option<usize>,
    #[pyo3(get)]
    pub exit_price: Option<f64>,
}

impl Trade {
    pub fn new(direction: TradeDirection) -> Self {
        return Trade {
            direction,
            is_closed: false,
            entry_price: None,
            entry_tick: None,
            exit_price: None,
            exit_tick: None,
        };
    }

    pub fn pnl(&self, fill_size: f64, current_price: f64) -> f64 {
        return compute_trade_pnl(
            fill_size,
            self.entry_price.unwrap(),
            current_price,
            self.direction == TradeDirection::Long,
        );
    }

    pub fn is_at_entry(&self, current_tick: usize) -> bool {
        return self.entry_tick.is_some() && self.entry_tick.unwrap() == current_tick;
    }

    pub fn is_at_exit(&self, current_tick: usize) -> bool {
        return self.exit_tick.is_some() && self.exit_tick.unwrap() == current_tick;
    }

    pub fn is_active(&self) -> bool {
        return self.entry_tick.is_some() && !self.is_closed;
    }

    pub fn to_colored_string(&self, current_tick: usize) -> ColoredString {
        if !self.is_closed {
            if self.direction == TradeDirection::Long {
                return "▲ [LONG]".green().bold();
            } else {
                return "▼ [SHORT]".red().bold();
            }
        } else if current_tick == self.exit_tick.unwrap() {
            if self.direction == TradeDirection::Long {
                return format!("{} {}", "▼".red(), "[LONG_EXIT]".green()).bold();
            } else {
                return format!("{} {}", "▲".green(), "[SHORT_EXIT]".red()).bold();
            }
        }
        return "No Trade".bright_black();
    }

    pub fn get_triangle_colored_string(&self, current_tick: usize) -> ColoredString {
        if !self.is_closed && self.entry_tick.is_some() && self.entry_tick.unwrap() == current_tick
        {
            if self.direction == TradeDirection::Long {
                return "▲".green().bold();
            } else {
                return "▼".red().bold();
            }
        } else if self.exit_tick.is_some() && current_tick == self.exit_tick.unwrap() {
            if self.direction == TradeDirection::Long {
                return "▼".red().bold();
            } else {
                return "▲".green().bold();
            }
        }
        if self.exit_tick.is_none() {
            if self.direction == TradeDirection::Long {
                return "—".green().bold();
            } else {
                return "—".red().bold();
            }
        }
        if self.direction == TradeDirection::Long {
            return "—".black().bold();
        } else {
            return "—".black().bold();
        }
    }
}

pub fn compute_trade_pnl(
    fill_size: f64,
    fill_price: f64,
    current_price: f64,
    is_long: bool,
) -> f64 {
    let multiplier = if is_long { 1.0 } else { -1.0 };
    return (current_price - fill_price) * fill_size * multiplier;
}

pub fn compute_fill_size(equity: f64, current_price: f64) -> f64 {
    // if equity <= 0.0 || current_price <= 0.0 {
    //     return 0.0;
    // }
    if current_price == 0.0 {
        return 0.0;
    }
    return equity / current_price;
}

pub fn compute_pnl(current_equity: f64, initial_equity: f64) -> f64 {
    return current_equity - initial_equity;
}

pub fn compute_return(current_equity: f64, previous_equity: f64) -> f64 {
    if previous_equity == 0.0 || current_equity == 0.0 {
        return 0.0;
    }
    if current_equity < 0.0 || previous_equity < 0.0 {
        return 0.0;
    }
    // if current_equity < 0.0 && previous_equity < 0.0 {
    //     return 1.0 - (current_equity / previous_equity);
    // }
    let returns = (current_equity / previous_equity) - 1.0;
    // if returns > 1.0 || returns < -1.0 {
    //     return 0.0;
    // }
    return returns;
}

pub fn signed_log(value: f64) -> f64 {
    let sign = value.signum();
    let value = value.abs() + 1.0;
    return sign * value.log10();
}
