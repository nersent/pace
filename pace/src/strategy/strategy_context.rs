use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

use crate::components::component_context::ComponentContext;

use super::trade::{fill_size, Trade, TradeDirection};

pub struct StrategyOnTradeEntryEvent {
    pub trade: Trade,
}

pub struct StrategyOnTradeExitEvent {
    pub trade: Trade,
    pub pnl: f64,
}

pub struct StrategyContextEvents {
    pub on_trade_entry: Option<StrategyOnTradeEntryEvent>,
    pub on_trade_exit: Option<StrategyOnTradeExitEvent>,
}

#[derive(Clone, Copy, Debug)]
pub struct StrategyContextConfig {
    /**
    Enables an additional calculation on bar close, allowing market orders to enter on the same tick the order is placed
    */
    pub on_bar_close: bool,
    pub continous: bool,
    pub initial_capital: f64,
    pub buy_with_equity: bool,
}

impl Default for StrategyContextConfig {
    fn default() -> Self {
        return Self {
            buy_with_equity: false,
            continous: true,
            on_bar_close: false,
            initial_capital: 1000.0,
        };
    }
}

pub struct StrategyState {
    pub config: StrategyContextConfig,
    pub trades: Vec<Trade>,
    pub unfilled_trade_direction: Option<TradeDirection>,
    pub events: StrategyContextEvents,
    pub equity: f64,
    pub net_profit: f64,
    pub open_profit: f64,
}

/// Manages trades and provides data for all strategy components.
pub struct StrategyContext {
    pub ctx: Context,
    pub initial_capital: f64,
    state: Rc<RefCell<StrategyState>>,
}

impl StrategyContext {
    pub fn new(ctx: Context, config: StrategyContextConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            state: Rc::new(RefCell::new(StrategyState {
                trades: Vec::new(),
                unfilled_trade_direction: None,
                events: StrategyContextEvents {
                    on_trade_entry: None,
                    on_trade_exit: None,
                },
                equity: config.initial_capital,
                net_profit: 0.0,
                open_profit: 0.0,
                config,
            })),
            initial_capital: config.initial_capital,
        };
    }

    pub fn state(&self) -> Ref<StrategyState> {
        return self.state.as_ref().borrow();
    }

    pub fn clone(&self) -> Self {
        return Self {
            ctx: self.ctx.clone(),
            state: Rc::clone(&self.state),
            initial_capital: self.initial_capital,
        };
    }

    pub fn next(&mut self, direction: Option<TradeDirection>) {
        let tick = self.ctx.bar_index();
        let open = self.ctx.open();
        let close = self.ctx.close();

        let mut state = self.state.as_ref().borrow_mut();

        if state.config.on_bar_close {
            state.unfilled_trade_direction = direction;
        }

        state.events.on_trade_entry = None;
        state.events.on_trade_exit = None;

        if let Some(unfilled_trade_direction) = state.unfilled_trade_direction {
            let is_continous = state.config.continous;

            let mut close_trade = false;
            let mut create_new_trade = false;

            let orderbook_price = if state.config.on_bar_close {
                close
            } else {
                open
            };

            if let Some(last_trade) = state.trades.last_mut() {
                let is_same_direction = last_trade.direction == unfilled_trade_direction;

                close_trade = !is_same_direction && !last_trade.is_closed;

                if is_continous {
                    create_new_trade = !is_same_direction && close_trade;
                } else {
                    create_new_trade = last_trade.is_closed
                        && (is_same_direction || !is_same_direction && !close_trade);
                }

                if close_trade {
                    last_trade.exit_price = orderbook_price;
                    last_trade.exit_tick = Some(tick);
                    last_trade.is_closed = true;

                    let trade_pnl = last_trade.pnl(last_trade.exit_price.unwrap());

                    state.events.on_trade_exit = Some(StrategyOnTradeExitEvent {
                        trade: *last_trade,
                        pnl: trade_pnl,
                    });

                    state.net_profit += trade_pnl;
                    state.open_profit = 0.0;
                }
            } else {
                create_new_trade = true;
            }

            if create_new_trade {
                let mut trade = Trade::new(unfilled_trade_direction);

                trade.fill_size = Some(1.0);

                if state.config.buy_with_equity {
                    let equity =
                        state.config.initial_capital + state.net_profit + state.open_profit;

                    trade.fill_size = Some(fill_size(equity, orderbook_price.unwrap()));
                }

                trade.entry_price = orderbook_price;
                trade.entry_tick = Some(tick);

                state.trades.push(trade);
                state.events.on_trade_entry = Some(StrategyOnTradeEntryEvent { trade: trade });
            }

            state.unfilled_trade_direction = None;
        }

        if !state.config.on_bar_close {
            state.unfilled_trade_direction = direction;
        }

        if let Some(last_trade) = state.trades.last_mut() {
            if !last_trade.is_closed {
                state.open_profit = last_trade.pnl(close.unwrap());
            }
        }

        state.equity = state.config.initial_capital + state.net_profit + state.open_profit;
    }
}
