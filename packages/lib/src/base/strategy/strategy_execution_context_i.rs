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
            if self.trades.is_empty() {
                self.trades.push(Trade::new(filled_order.direction));
            }

            let current_trade = self.trades.last_mut().unwrap();

            if self.config.continous {
                if current_trade.entry_tick.is_none()
                    && current_trade.direction == filled_order.direction
                {
                    current_trade.entry_price = filled_order.fill_price;
                    current_trade.entry_tick = filled_order.fill_tick;
                } else if current_trade.entry_price.is_some() && !current_trade.is_closed {
                    current_trade.is_closed = true;
                    current_trade.exit_price = filled_order.fill_price;
                    current_trade.exit_tick = filled_order.fill_tick;

                    let mut opposite_direction_trade = Trade::new(filled_order.direction);
                    opposite_direction_trade.entry_price = filled_order.fill_price;
                    opposite_direction_trade.entry_tick = filled_order.fill_tick;

                    self.trades.push(opposite_direction_trade);
                }
            } else {
                if current_trade.entry_price.is_some() && !current_trade.is_closed {
                    current_trade.is_closed = true;
                    current_trade.exit_price = filled_order.fill_price;
                    current_trade.exit_tick = filled_order.fill_tick;
                } else {
                    if current_trade.entry_tick.is_none() {
                        current_trade.entry_price = filled_order.fill_price;
                        current_trade.entry_tick = filled_order.fill_tick;
                    } else if current_trade.is_closed {
                        let mut trade = Trade::new(filled_order.direction);
                        trade.entry_price = filled_order.fill_price;
                        trade.entry_tick = filled_order.fill_tick;
                        self.trades.push(trade);
                    }
                }

                // if current_trade.direction == filled_order.direction {
                //     if current_trade.entry_tick.is_none() {
                //         current_trade.entry_price = filled_order.fill_price;
                //         current_trade.entry_tick = filled_order.fill_tick;
                //     } else if current_trade.is_closed {
                //         let mut trade = Trade::new(filled_order.direction);
                //         trade.entry_price = filled_order.fill_price;
                //         trade.entry_tick = filled_order.fill_tick;
                //         self.trades.push(trade);
                //     }
                // } else if current_trade.entry_price.is_some() && !current_trade.is_closed {
                //     current_trade.is_closed = true;
                //     current_trade.exit_price = filled_order.fill_price;
                //     current_trade.exit_tick = filled_order.fill_tick;
                // }
            }
        }

        return self.trades.last();
    }
}
