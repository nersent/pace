use colored::{ColoredString, Colorize};
use polars::export::chrono::format;

use super::{action::StrategyActionKind, strategy_utils::{compute_pnl, compute_trade_pnl}};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TradeDirection {
    Long,
    Short,
}

impl TradeDirection {
    pub fn get_opposite(&self) -> Self {
        return match self {
            TradeDirection::Long => TradeDirection::Short,
            TradeDirection::Short => TradeDirection::Long,
        };
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Trade {
    pub direction: TradeDirection,
    pub is_closed: bool,
    pub entry_price: Option<f64>,
    pub entry_tick: Option<usize>,
    pub exit_price: Option<f64>,
    pub exit_tick: Option<usize>,
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

    pub fn pnl(&self, fill_size: f64, current_price: f64) -> Option<f64> {
        match self.entry_price {
            Some(entry_price) => {
                return Some(compute_trade_pnl(
                    fill_size,
                    entry_price,
                    current_price,
                    self.direction == TradeDirection::Long,
                ));
            }
            _ => return None,
        }
    }

    pub fn to_colored_string(&self) -> ColoredString {
        if !self.is_closed {
            if self.direction == TradeDirection::Long {
                return "▲ [LONG]".green().bold();
            } else {
                return "▼ [SHORT]".red().bold();
            }
        } else {
            if self.direction == TradeDirection::Long {
                return format!("{} {}", "▼".red(), "[LONG_EXIT]".green()).bold();
            } else {
                return format!("{} {}", "▲".green(), "[SHORT_EXIT]".red()).bold();
            }
        }
    }
}
