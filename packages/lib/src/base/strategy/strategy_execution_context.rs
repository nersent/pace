use crate::base::components::{
    component_context::ComponentContext, component_default::ComponentDefault,
};

use super::{
    orderbook::{OrderBook, OrderBookConfig},
    trade::{Trade, TradeDirection},
};

pub struct StrategyExecutionContextConfig {
    /**
    Enables an additional calculation on bar close, allowing market orders to enter on the same tick the order is placed
    */
    pub on_bar_close: bool,
    pub slippage: usize,
}

impl ComponentDefault for StrategyExecutionContextConfig {
    fn default(ctx: ComponentContext) -> Self {
        return StrategyExecutionContextConfig {
            on_bar_close: false,
            slippage: 1,
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
                    slippage: config.slippage,
                },
            ),
            config,
            current_trade: None,
        };
    }

    pub fn next(&mut self, trade: Option<TradeDirection>) -> Option<&Trade> {
        self.ctx.assert();

        if let Some(trade) = trade {
            if self.current_trade.is_none() {
                self.current_trade = Some(Trade::new(trade));
                self.orderbook.place(trade);
            } else {
                let mut current_trade = self.current_trade.as_mut().unwrap();
                if current_trade.direction != trade {
                    self.orderbook.place(trade);
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
                self.trades.push(*current_trade);
                self.current_trade = None;
                return self.trades.last();
            }
        }

        return self.current_trade.as_ref();
    }
}
