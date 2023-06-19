use std::time::Duration;

use super::utils::trade_profit;

#[derive(Debug, PartialEq, Clone, Copy, FromPrimitive)]
pub enum TradeDirection {
    Long = 1,
    Short = -1,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Trade {
    // pub id: usize,
    /// the direction and the number of contracts traded in the closed trade. If the value is > 0, the market position was long. If the value is < 0, the market position was short.
    pub size: f64,
    pub direction: TradeDirection,
    pub closed: bool,
    //
    pub entry_id: Option<String>,
    pub entry_bar_index: Option<usize>,
    pub entry_price: f64,
    pub entry_time: Option<Duration>,
    pub entry_comment: Option<String>,
    pub exit_comment: Option<String>,
    //
    pub exit_id: Option<String>,
    pub exit_bar_index: Option<usize>,
    pub exit_price: f64,
    pub exit_time: Option<Duration>,
    /// The profit/loss of the trade. Losses are expressed as negative values.
    pub profit: f64,
    // @TODO
    // pub commision: f64,
}

impl Trade {
    pub fn new(direction: TradeDirection, size: f64) -> Self {
        assert!(size > 0.0, "Trade size must be greater than 0");
        return Self {
            size,
            direction,
            closed: false,
            //
            entry_id: None,
            entry_bar_index: None,
            entry_price: f64::NAN,
            entry_time: None,
            entry_comment: None,
            //
            exit_id: None,
            exit_bar_index: None,
            exit_price: f64::NAN,
            exit_time: None,
            exit_comment: None,
            //
            profit: 0.0,
        };
    }

    pub fn get_directional_size(&self) -> f64 {
        if self.direction == TradeDirection::Long {
            return self.size;
        } else {
            return -self.size;
        }
    }

    pub fn entry(&mut self, entry_price: f64, entry_id: String, entry_bar_index: usize) {
        assert!(
            !self.closed,
            "Cannot entry a trade that has already been closed"
        );
        assert!(
            self.entry_id.is_none(),
            "Cannot entry a trade that has already been entered"
        );
        self.entry_id = Some(entry_id);
        self.entry_bar_index = Some(entry_bar_index);
        self.entry_price = entry_price;
        self.entry_time = None;
    }

    pub fn update_profit(&mut self, price: f64) {
        assert!(
            !self.closed,
            "Cannot update profit on a trade that has already been closed"
        );
        assert!(
            self.entry_id.is_some(),
            "Cannot update profit on a trade that has not been entered"
        );
        let pnl = trade_profit(
            self.size,
            self.entry_price,
            price,
            self.direction == TradeDirection::Long,
        );
        self.profit = pnl;
    }

    pub fn close(&mut self, exit_price: f64, exit_id: String, exit_bar_index: usize) {
        assert!(
            !self.closed,
            "Cannot close a trade that has already been closed"
        );
        self.update_profit(exit_price);
        self.exit_price = exit_price;
        self.exit_id = Some(exit_id);
        self.exit_bar_index = Some(exit_bar_index);
        self.closed = true;
    }

    pub fn set_size(&mut self, size: f64) {
        assert!(size > 0.0, "Trade size must be greater than 0");
        self.size = size;
    }
}
