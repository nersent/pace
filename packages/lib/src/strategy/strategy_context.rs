use crate::components::component_context::ComponentContext;

use super::{
    action::StrategyActionKind,
    orderbook::{Order, OrderBook, OrderBookConfig},
    trade::Trade,
};

pub struct StrategyContextConfig {
    /**
    Enables an additional calculation on bar close, allowing market orders to enter on the same tick the order is placed
    */
    pub on_bar_close: bool,
}

pub struct StrategyContext {
    pub config: StrategyContextConfig,
    pub trades: Vec<Trade>,
    current_trade: Option<Trade>,
    ctx: ComponentContext,
    orderbook: OrderBook,
}

impl StrategyContext {
    pub fn new(ctx: ComponentContext, config: StrategyContextConfig) -> Self {
        return StrategyContext {
            config,
            ctx: ctx.clone(),
            trades: Vec::new(),
            orderbook: OrderBook::new(ctx.clone(), OrderBookConfig { slippage: 1 }),
            current_trade: None,
        };
    }

    pub fn next(&mut self, action: StrategyActionKind) -> Option<Trade> {
        self.ctx.assert();
        let direction = action.to_direction();

        if let Some(direction) = direction {
            if self.current_trade.is_none() {
                self.current_trade = Some(Trade::new(direction));
                self.orderbook.place(direction);
            } else {
                let mut current_trade = self.current_trade.as_mut().unwrap();
                if current_trade.direction != direction {
                    self.orderbook.place(direction);
                }
            }
        }

        let ctx = self.ctx.get();
        let filled_order = self.orderbook.next();

        if let Some(filled_order) = filled_order {
            assert!(self.current_trade.is_some(), "No trade");
            let mut current_trade = self.current_trade.as_mut().unwrap();

            if current_trade.entry_tick.is_none() {
                current_trade.entry_price = filled_order.fill_price;
                current_trade.entry_tick = filled_order.fill_tick;
            } else {
                current_trade.exit_price = filled_order.fill_price;
                current_trade.exit_tick = filled_order.fill_tick;
                current_trade.is_closed = true;
                let current_trade = current_trade.clone();
                self.trades.push(current_trade);
                self.current_trade = None;
                return Some(current_trade);
            }
        }

        return self.current_trade;
    }
}
