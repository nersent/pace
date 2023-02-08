use super::trade::TradeDirection;

pub trait RunnableStrategy {
    fn run(&mut self) -> Option<TradeDirection>;
}
