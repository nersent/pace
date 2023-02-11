use crate::base::{
    components::{component_context::ComponentContext, component_default::ComponentDefault},
    strategy::orderbook::Order,
};

use super::{
    orderbook::{OrderBook, OrderBookConfig},
    trade::{Trade, TradeDirection},
};

pub struct StrategyContextConfig {
    /**
    Enables an additional calculation on bar close, allowing market orders to enter on the same tick the order is placed
    */
    pub on_bar_close: bool,
    pub continous: bool,
    pub initial_capital: f64,
}

impl ComponentDefault for StrategyContextConfig {
    fn default(ctx: ComponentContext) -> Self {
        return StrategyContextConfig {
            on_bar_close: false,
            continous: true,
            initial_capital: 1000.0,
        };
    }
}

pub struct StrategyContext {
    pub config: StrategyContextConfig,
    pub trades: Vec<Trade>,
    ctx: ComponentContext,
    unfilled_trade_direction: Option<TradeDirection>,
}

impl StrategyContext {
    pub fn new(ctx: ComponentContext, config: StrategyContextConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            trades: Vec::new(),
            config,
            unfilled_trade_direction: None,
        };
    }

    pub fn next(&mut self, direction: Option<TradeDirection>) {
        self.ctx.on_next();

        let ctx = self.ctx.get();
        let tick = ctx.current_tick;

        if self.config.on_bar_close {
            self.unfilled_trade_direction = direction;
        }

        if let Some(unfilled_trade_direction) = self.unfilled_trade_direction {
            let mut last_trade = self.trades.last_mut();

            let orderbook_price = if self.config.on_bar_close {
                ctx.close()
            } else {
                ctx.open()
            };

            if let Some(last_trade) = last_trade {
                if last_trade.direction != unfilled_trade_direction {
                    let mut closed_now = false;

                    if !last_trade.is_closed {
                        last_trade.exit_price = orderbook_price;
                        last_trade.exit_tick = Some(tick);
                        last_trade.is_closed = true;
                        closed_now = true;
                    }

                    println!(
                        "[{tick}] XDDDD {:?} | {:?}",
                        last_trade.clone(),
                        unfilled_trade_direction
                    );

                    if self.config.continous && closed_now
                        || !self.config.continous && !closed_now && last_trade.is_closed
                    {
                        let mut new_trade = Trade::new(unfilled_trade_direction);
                        new_trade.entry_price = orderbook_price;
                        new_trade.entry_tick = Some(tick);

                        self.trades.push(new_trade);
                    }
                } else if !self.config.continous && last_trade.is_closed {
                    println!(
                        "[{tick}] AHA {:?} | {:?}",
                        last_trade.clone(),
                        unfilled_trade_direction
                    );
                    let mut new_trade = Trade::new(unfilled_trade_direction);
                    new_trade.entry_price = orderbook_price;
                    new_trade.entry_tick = Some(tick);

                    self.trades.push(new_trade);
                }
            } else {
                let mut trade = Trade::new(unfilled_trade_direction);
                trade.entry_price = orderbook_price;
                trade.entry_tick = Some(tick);
                self.trades.push(trade);
            }

            self.unfilled_trade_direction = None;
        }

        if !self.config.on_bar_close {
            self.unfilled_trade_direction = direction;
        }

        println!("[{}]: {:?}", tick, self.trades.last());

        // if let Some(direction) = direction {
        //     self.orderbook.place(Order::new(direction));
        // }

        // let orderbook_price = if self.config.on_bar_close {
        //     ctx.close()
        // } else {
        //     ctx.open()
        // };

        // let filled_orders = self.orderbook.next(orderbook_price.unwrap());

        // assert!(filled_orders.len() <= 1, "Max one order should be filled");

        // let filled_order = filled_orders.first();

        // if let Some(filled_order) = filled_order {
        //     if self.trades.is_empty() {
        //         self.trades.push(Trade::new(filled_order.direction));
        //     }

        //     let current_trade = self.trades.last_mut().unwrap();

        //     if self.config.continous {
        //         if current_trade.entry_tick.is_none()
        //             && current_trade.direction == filled_order.direction
        //         {
        //             current_trade.entry_price = filled_order.fill_price;
        //             current_trade.entry_tick = filled_order.fill_tick;
        //         } else if current_trade.entry_price.is_some()
        //             && !current_trade.is_closed
        //             && current_trade.direction != filled_order.direction
        //         {
        //             current_trade.is_closed = true;
        //             current_trade.exit_price = filled_order.fill_price;
        //             current_trade.exit_tick = filled_order.fill_tick;

        //             let mut opposite_direction_trade = Trade::new(filled_order.direction);
        //             opposite_direction_trade.entry_price = filled_order.fill_price;
        //             opposite_direction_trade.entry_tick = filled_order.fill_tick;

        //             self.trades.push(opposite_direction_trade);
        //         }
        //     } else {
        //         if current_trade.entry_price.is_some() && !current_trade.is_closed {
        //             if current_trade.direction != filled_order.direction {
        //                 current_trade.is_closed = true;
        //                 current_trade.exit_price = filled_order.fill_price;
        //                 current_trade.exit_tick = filled_order.fill_tick;
        //             }
        //         } else {
        //             if current_trade.entry_tick.is_none() {
        //                 current_trade.entry_price = filled_order.fill_price;
        //                 current_trade.entry_tick = filled_order.fill_tick;
        //             } else if current_trade.is_closed {
        //                 let mut trade = Trade::new(filled_order.direction);
        //                 trade.entry_price = filled_order.fill_price;
        //                 trade.entry_tick = filled_order.fill_tick;
        //                 self.trades.push(trade);
        //             }
        //         }
        //     }
        // }
    }
}
