use crate::base::components::{
    component_context::ComponentContext, component_default::ComponentDefault,
};

use super::{
    orderbook_i::{Order, OrderBook, OrderBookConfig},
    trade::{Trade, TradeDirection},
};

pub struct StrategyExecutionContextConfig {
    /**
    Enables an additional calculation on bar close, allowing market orders to enter on the same tick the order is placed
    */
    pub on_bar_close: bool,
    pub continous: bool,
}

impl ComponentDefault for StrategyExecutionContextConfig {
    fn default(ctx: ComponentContext) -> Self {
        return StrategyExecutionContextConfig {
            on_bar_close: false,
            continous: true,
        };
    }
}

pub struct StrategyExecutionContext {
    pub config: StrategyExecutionContextConfig,
    pub trades: Vec<Trade>,
    current_trade: Option<Trade>,
    ctx: ComponentContext,
    orderbook: OrderBook,
}

impl StrategyExecutionContext {
    pub fn new(ctx: ComponentContext, config: StrategyExecutionContextConfig) -> Self {
        return StrategyExecutionContext {
            ctx: ctx.clone(),
            trades: Vec::new(),
            orderbook: OrderBook::new(
                ctx.clone(),
                OrderBookConfig {
                    slippage: if config.on_bar_close { 0 } else { 1 },
                },
            ),
            config,
            current_trade: None,
        };
    }

    pub fn next(&mut self, direction: Option<TradeDirection>) -> Option<&Trade> {
        self.ctx.assert();
        let ctx = self.ctx.get();

        if let Some(direction) = direction {
            self.orderbook.place(Order::new(direction));
        }

        let orderbook_price = if self.config.on_bar_close {
            ctx.close()
        } else {
            ctx.open()
        };

        let filled_orders = self.orderbook.next(orderbook_price.unwrap());

        assert!(filled_orders.len() <= 1, "Max one order should be filled");

        let filled_order = filled_orders.first();

        if let Some(filled_order) = filled_order {
            if self.current_trade.is_none() {
                self.current_trade = Some(Trade::new(filled_order.direction));
            }

            let current_trade = self.current_trade.as_mut().unwrap();

            if current_trade.direction == filled_order.direction {
                current_trade.entry_price = filled_order.fill_price;
                current_trade.entry_tick = filled_order.fill_tick;
            } else {
                current_trade.is_closed = true;
                current_trade.exit_price = filled_order.fill_price;
                current_trade.exit_tick = filled_order.fill_tick;
                self.trades.push(*current_trade);

                let mut current_trade = Trade::new(filled_order.direction);
                current_trade.entry_price = filled_order.fill_price;
                current_trade.entry_tick = filled_order.fill_tick;

                self.current_trade = Some(current_trade);

                return self.current_trade.as_ref();
            }
        }

        return self.current_trade.as_ref();
    }
}
