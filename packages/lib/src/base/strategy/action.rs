#[derive(Debug, PartialEq, Clone)]
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
}
