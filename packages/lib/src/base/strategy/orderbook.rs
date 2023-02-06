use std::collections::HashMap;

use crate::base::components::component_context::ComponentContext;

use super::trade::TradeDirection;

#[derive(Debug, Clone, Copy)]
pub struct Order {
    pub direction: TradeDirection,
    pub entry_tick: usize,
    pub is_filled: bool,
    pub fill_tick: Option<usize>,
    pub fill_price: Option<f64>,
}

impl Order {
    pub fn new(direction: TradeDirection, entry_tick: usize) -> Self {
        return Order {
            direction,
            entry_tick,
            is_filled: false,
            fill_tick: None,
            fill_price: None,
        };
    }
}

pub struct OrderBookConfig {
    pub slippage: usize,
}

pub struct OrderBook {
    pub config: OrderBookConfig,
    ctx: ComponentContext,
    current_order: Option<Order>,
}

impl OrderBook {
    pub fn new(ctx: ComponentContext, config: OrderBookConfig) -> Self {
        return OrderBook {
            config,
            ctx,
            current_order: None,
        };
    }

    pub fn place(&mut self, direction: TradeDirection) {
        let ctx = self.ctx.get();
        let order = Order::new(direction, ctx.current_tick);
        self.current_order = Some(order);
    }

    pub fn next(&mut self) -> Option<&mut Order> {
        self.ctx.assert();
        let current_order = self.current_order.as_ref();

        if current_order.is_none() || current_order.unwrap().is_filled {
            return None;
        }

        let mut current_order = self.current_order.as_mut().unwrap();
        let ctx = self.ctx.get();
        let tick = ctx.current_tick;

        if tick - current_order.entry_tick < self.config.slippage {
            return None;
        }

        current_order.is_filled = true;
        current_order.fill_tick = Some(tick);
        current_order.fill_price = ctx.open();

        return Some(current_order);
    }
}
