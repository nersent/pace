use super::trade::StrategyTradeDirection;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum StrategyActionKind {
    None,
    Long,
    Short,
}

impl StrategyActionKind {
    pub fn to_f64(&self) -> f64 {
        return match self {
            StrategyActionKind::None => 0.0,
            StrategyActionKind::Long => 1.0,
            StrategyActionKind::Short => -1.0,
        };
    }

    pub fn to_direction(&self) -> StrategyTradeDirection {
        return match self {
            StrategyActionKind::None => panic!("Cannot convert None to direction"),
            StrategyActionKind::Long => StrategyTradeDirection::Long,
            StrategyActionKind::Short => StrategyTradeDirection::Short,
        };
    }
}
