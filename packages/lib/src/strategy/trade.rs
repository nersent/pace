use super::action::StrategyActionKind;

#[derive(Debug, PartialEq, Clone)]
pub enum StrategyTradeDirection {
    Long,
    Short,
}

impl StrategyTradeDirection {
    pub fn get_opposite(&self) -> Self {
        return match self {
            StrategyTradeDirection::Long => StrategyTradeDirection::Short,
            StrategyTradeDirection::Short => StrategyTradeDirection::Long,
        };
    }
}

#[derive(Debug)]
pub struct StrategyTrade {
    pub direction: StrategyTradeDirection,
    pub is_filled: bool,
    pub is_closed: bool,
    pub fill_tick: Option<usize>,
    pub fill_price: Option<f64>,
    pub close_tick: Option<usize>,
    pub close_price: Option<f64>,
}
