use std::collections::HashMap;

use crate::base::components::component_context::ComponentContext;

use super::trade::TradeDirection;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Order {
    pub direction: TradeDirection,
    pub place_tick: Option<usize>,
    pub is_filled: bool,
    pub fill_tick: Option<usize>,
    pub fill_price: Option<f64>,
}

impl Order {
    pub fn new(direction: TradeDirection) -> Self {
        return Order {
            direction,
            place_tick: None,
            is_filled: false,
            fill_tick: None,
            fill_price: None,
        };
    }
}

pub struct OrderBookConfig {
    pub slippage: usize,
}

impl Default for OrderBookConfig {
    fn default() -> Self {
        return OrderBookConfig { slippage: 1 };
    }
}

pub struct OrderBook {
    pub config: OrderBookConfig,
    pub orders: Vec<Order>,
    ctx: ComponentContext,
}

impl OrderBook {
    pub fn new(ctx: ComponentContext, config: OrderBookConfig) -> Self {
        return OrderBook {
            config,
            ctx,
            orders: Vec::new(),
        };
    }

    pub fn place(&mut self, order: Order) {
        let tick = self.ctx.get().current_tick;
        let mut order = order;
        order.place_tick = Some(tick);
        self.orders.push(order);
    }

    // Returns filled orders
    pub fn next(&mut self, price: f64) -> Vec<Order> {
        let ctx = self.ctx.get();
        let tick = ctx.current_tick;

        let (filled_orders, unfilled_orders): (Vec<Order>, Vec<Order>) =
            self.orders.iter().partition(|order| {
                return order.is_filled || tick - order.place_tick.unwrap() >= self.config.slippage;
            });

        self.orders = unfilled_orders;

        let filled_orders = filled_orders
            .iter()
            .map(|order| {
                let mut order = order.clone();
                order.fill_tick = Some(tick);
                order.fill_price = Some(price);
                order.is_filled = true;
                return order;
            })
            .collect::<Vec<Order>>();

        return filled_orders;

        // self.orders = self
        //     .orders
        //     .iter_mut()
        //     .filter(|order| {
        //         let ctx = self.ctx.get();
        //         let tick = ctx.current_tick;
        //         return order.is_filled || tick - order.place_tick >= self.config.slippage;
        //     })
        //     .map(|order| {
        //         let ctx = self.ctx.get();
        //         let tick = ctx.current_tick;
        //         return order;
        //     })
        //     .collect();

        // return None;
        // self.ctx.assert();
        // let current_order = self.current_order.as_ref();

        // if current_order.is_none() || current_order.unwrap().is_filled {
        //     return None;
        // }

        // let mut current_order = self.current_order.as_mut().unwrap();
        // let ctx = self.ctx.get();
        // let tick = ctx.current_tick;

        // if tick - current_order.entry_tick < self.config.slippage {
        //     return None;
        // }

        // current_order.is_filled = true;
        // current_order.fill_tick = Some(tick);
        // current_order.fill_price = ctx.open();

        // return Some(current_order);
    }
}
