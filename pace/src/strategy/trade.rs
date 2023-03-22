use colored::{ColoredString, Colorize};

#[derive(Debug, PartialEq, Clone, Copy)]
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
pub struct Trade {
    pub direction: TradeDirection,
    pub is_closed: bool,
    pub entry_tick: Option<usize>,
    pub entry_price: Option<f64>,
    pub exit_tick: Option<usize>,
    pub exit_price: Option<f64>,
    pub fill_size: Option<f64>,
    pub pnl: f64,
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
            fill_size: None,
            pnl: 0.0,
        };
    }

    pub fn pnl(&self, current_price: f64) -> f64 {
        return trade_pnl(
            self.fill_size.unwrap(),
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

pub fn trade_pnl(fill_size: f64, fill_price: f64, current_price: f64, is_long: bool) -> f64 {
    let multiplier = if is_long { 1.0 } else { -1.0 };
    return (current_price - fill_price) * fill_size * multiplier;
}

pub fn fill_size(equity: f64, current_price: f64) -> f64 {
    if equity <= 0.0 || current_price <= 0.0 {
        return 0.0;
    }
    return equity / current_price;
}
