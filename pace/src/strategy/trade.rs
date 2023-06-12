use colored::{ColoredString, Colorize};

#[derive(Debug, PartialEq, Clone, Copy, FromPrimitive)]
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

impl Into<StrategySignal> for TradeDirection {
    fn into(self) -> StrategySignal {
        return match self {
            TradeDirection::Long => StrategySignal::Long,
            TradeDirection::Short => StrategySignal::Short,
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
    pub entry_price: f64,
    pub exit_tick: Option<usize>,
    pub exit_price: f64,
    pub fill_size: f64,
    pub pnl: f64,
    pub size: f64,
}

impl Trade {
    pub fn new(direction: TradeDirection) -> Self {
        return Trade {
            direction,
            is_closed: false,
            entry_price: f64::NAN,
            entry_tick: None,
            exit_price: f64::NAN,
            exit_tick: None,
            fill_size: f64::NAN,
            pnl: 0.0,
            size: f64::NAN,
        };
    }

    pub fn pnl(&self, current_price: f64) -> f64 {
        return trade_pnl(
            self.fill_size,
            self.entry_price,
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

#[derive(PartialEq, Copy, Debug, Clone)]
pub enum StrategySignal {
    Hold,
    Long,
    Short,
    LongEntry,
    ShortEntry,
    LongExit,
    ShortExit,
    Exit,
    Sized(f64),
    Dynamic(f64),
}

impl StrategySignal {
    pub fn continous(self) -> Option<TradeDirection> {
        return match self {
            StrategySignal::Long => Some(TradeDirection::Long),
            StrategySignal::Short => Some(TradeDirection::Short),
            StrategySignal::LongEntry => Some(TradeDirection::Long),
            StrategySignal::ShortEntry => Some(TradeDirection::Short),
            StrategySignal::LongExit => Some(TradeDirection::Short),
            StrategySignal::ShortExit => Some(TradeDirection::Long),
            _ => None,
        };
    }

    pub fn is_explicit_entry(self) -> bool {
        return self == StrategySignal::LongEntry || self == StrategySignal::ShortEntry;
    }

    pub fn is_explicit_exit(self) -> bool {
        return self == StrategySignal::LongExit || self == StrategySignal::ShortExit;
    }

    // pub fn to_colored_string(&self, current_tick: usize) -> ColoredString {
    //     if !self.is_closed {
    //         if self.direction == TradeDirection::Long {
    //             return "▲ [LONG]".green().bold();
    //         } else {
    //             return "▼ [SHORT]".red().bold();
    //         }
    //     } else if current_tick == self.exit_tick.unwrap() {
    //         if self.direction == TradeDirection::Long {
    //             return format!("{} {}", "▼".red(), "[LONG_EXIT]".green()).bold();
    //         } else {
    //             return format!("{} {}", "▲".green(), "[SHORT_EXIT]".red()).bold();
    //         }
    //     }
    //     return "No Trade".bright_black();
    // }

    // pub fn get_triangle_colored_string(&self, current_tick: usize) -> ColoredString {
    //     if !self.is_closed && self.entry_tick.is_some() && self.entry_tick.unwrap() == current_tick
    //     {
    //         if self.direction == TradeDirection::Long {
    //             return "▲".green().bold();
    //         } else {
    //             return "▼".red().bold();
    //         }
    //     } else if self.exit_tick.is_some() && current_tick == self.exit_tick.unwrap() {
    //         if self.direction == TradeDirection::Long {
    //             return "▼".red().bold();
    //         } else {
    //             return "▲".green().bold();
    //         }
    //     }
    //     if self.exit_tick.is_none() {
    //         if self.direction == TradeDirection::Long {
    //             return "—".green().bold();
    //         } else {
    //             return "—".red().bold();
    //         }
    //     }
    //     if self.direction == TradeDirection::Long {
    //         return "—".black().bold();
    //     } else {
    //         return "—".black().bold();
    //     }
    // }
}

impl Into<i32> for StrategySignal {
    fn into(self) -> i32 {
        return match self {
            StrategySignal::Hold => 0,
            StrategySignal::Long => 1,
            StrategySignal::Short => -1,
            StrategySignal::LongEntry => 2,
            StrategySignal::ShortEntry => -2,
            StrategySignal::LongExit => 3,
            StrategySignal::ShortExit => -3,
            StrategySignal::Exit => -4,
            _ => 0,
        };
    }
}

impl Into<f64> for StrategySignal {
    fn into(self) -> f64 {
        let value: i32 = self.into();
        return value as f64;
    }
}

impl From<i32> for StrategySignal {
    fn from(value: i32) -> Self {
        return match value {
            0 => StrategySignal::Hold,
            1 => StrategySignal::Long,
            -1 => StrategySignal::Short,
            2 => StrategySignal::LongEntry,
            -2 => StrategySignal::ShortEntry,
            3 => StrategySignal::LongExit,
            -3 => StrategySignal::ShortExit,
            -4 => StrategySignal::Exit,
            _ => StrategySignal::Hold,
        };
    }
}

impl From<f64> for StrategySignal {
    fn from(value: f64) -> Self {
        let value: i32 = value as i32;
        return StrategySignal::from(value);
    }
}

#[derive(Debug, Clone)]
pub struct SignalFixture {
    pub long_entries: Vec<usize>,
    pub short_entries: Vec<usize>,
    pub long_exits: Vec<usize>,
    pub short_exits: Vec<usize>,
}

impl SignalFixture {
    pub fn get(&self, tick: usize) -> StrategySignal {
        if self.long_entries.contains(&tick) {
            return StrategySignal::LongEntry;
        }
        if self.short_entries.contains(&tick) {
            return StrategySignal::ShortEntry;
        }
        if self.long_exits.contains(&tick) {
            return StrategySignal::LongExit;
        }
        if self.short_exits.contains(&tick) {
            return StrategySignal::ShortExit;
        }
        return StrategySignal::Hold;
    }
}
