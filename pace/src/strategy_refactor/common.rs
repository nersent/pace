use super::trade::TradeDirection;

#[derive(Debug)]
pub enum Qty {
    Default,
    Contracts(f64),
    EquityPct(f64),
    // Cash(f64),
}

#[derive(Debug)]
pub struct SignalOptions {
    /// The order identifier. It is possible to cancel or modify an order by referencing its identifier.
    pub id: Option<String>,
    ///  Order direction: TradeDirection::Long is for buy, 'strategy.short' is for TradeDirection::Short.
    pub direction: TradeDirection,
    /// Number of contracts/shares/lots/units to trade. The default value is 'NaN'.
    pub qty: Qty,
}

impl SignalOptions {
    pub fn new(direction: TradeDirection) -> Self {
        Self {
            id: None,
            direction,
            qty: Qty::Default,
        }
    }

    pub fn with_id(self, id: String) -> Self {
        Self {
            id: Some(id),
            ..self
        }
    }

    pub fn clear_id(self) -> Self {
        Self { id: None, ..self }
    }

    pub fn with_qty(self, qty: Qty) -> Self {
        Self { qty, ..self }
    }
}

#[derive(Debug)]
pub enum Signal {
    Entry(SignalOptions),
    Exit(SignalOptions),
    Order(SignalOptions),
}

impl Signal {
    pub fn as_signal_options(self) -> Option<SignalOptions> {
        return match self {
            Signal::Entry(options) => Some(options),
            Signal::Exit(options) => Some(options),
            Signal::Order(options) => Some(options),
            _ => None,
        };
    }
}
