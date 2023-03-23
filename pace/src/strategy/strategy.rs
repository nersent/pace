use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

use crate::core::{context::Context, incremental::Incremental};

use super::trade::{fill_size, Trade, TradeDirection};

pub struct StrategyOnTradeEntryEvent {
    pub trade: Trade,
}

pub struct StrategyOnTradeExitEvent {
    pub trade: Trade,
}

pub struct StrategyEvents {
    pub on_trade_entry: Option<StrategyOnTradeEntryEvent>,
    pub on_trade_exit: Option<StrategyOnTradeExitEvent>,
}

#[derive(Clone, Copy, Debug)]
pub struct StrategyConfig {
    /**
    Enables an additional calculation on bar close, allowing market orders to enter on the same tick the order is placed
    */
    pub on_bar_close: bool,
    pub continous: bool,
    pub initial_capital: f64,
    pub buy_with_equity: bool,
}

impl Default for StrategyConfig {
    fn default() -> Self {
        return Self {
            buy_with_equity: false,
            continous: true,
            on_bar_close: false,
            initial_capital: 1000.0,
        };
    }
}

/// Basic strategy metrics.
pub struct StrategyMetrics {
    /// Current equity (initial capital + net profit + open profit).
    /// Same as PineScript `strategy.equity`.
    pub equity: f64,
    /// The overall profit or loss. Same as PineScript `strategy.netprofit`.
    pub net_profit: f64,
    /// Current unrealized profit or loss for all open positions. Same as `strategy.openprofit`
    pub open_profit: f64,
    /// Total value of all completed winning trades. Same as PineScript `strategy.grossprofit`.
    pub gross_profit: f64,
    /// Total value of all completed losing trades. Same as PineScript `strategy.grossloss`.
    pub gross_loss: f64,
    /// Total number of closed trades. Same as PineScript `strategy.closedtrades`.
    pub closed_trades: usize,
    /// Total number of winning trades. Same as PineScript `strategy.wintrades`.
    pub winning_trades: usize,
    /// Total number of losing trades. Same as PineScript `strategy.losstrades`.
    pub losing_trades: usize,
    pub long_net_profit: f64,
    pub short_net_profit: f64,
}

impl StrategyMetrics {
    pub fn default(initial_capital: f64) -> Self {
        return Self {
            equity: initial_capital,
            net_profit: 0.0,
            open_profit: 0.0,
            closed_trades: 0,
            gross_loss: 0.0,
            gross_profit: 0.0,
            losing_trades: 0,
            winning_trades: 0,
            long_net_profit: 0.0,
            short_net_profit: 0.0,
        };
    }
}

/// Manages trades and provides data for all strategy components.
pub struct Strategy {
    pub ctx: Context,
    pub config: StrategyConfig,
    pub trades: Vec<Trade>,
    pub unfilled_trade_direction: Option<TradeDirection>,
    pub events: StrategyEvents,
    pub metrics: StrategyMetrics,
}

impl Strategy {
    pub fn new(ctx: Context, config: StrategyConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            trades: Vec::new(),
            unfilled_trade_direction: None,
            events: StrategyEvents {
                on_trade_entry: None,
                on_trade_exit: None,
            },
            metrics: StrategyMetrics::default(config.initial_capital),
            config,
        };
    }
}

impl Incremental<Option<TradeDirection>, ()> for Strategy {
    fn next(&mut self, direction: Option<TradeDirection>) {
        let tick = self.ctx.bar.index();
        let open = self.ctx.bar.open();
        let close = self.ctx.bar.close();

        if self.config.on_bar_close {
            self.unfilled_trade_direction = direction;
        }

        self.events.on_trade_entry = None;
        self.events.on_trade_exit = None;

        if let Some(unfilled_trade_direction) = self.unfilled_trade_direction {
            let is_continous = self.config.continous;

            let mut close_trade = false;
            let mut create_new_trade = false;

            let orderbook_price = if self.config.on_bar_close {
                close
            } else {
                open
            };

            if let Some(last_trade) = self.trades.last_mut() {
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
                    last_trade.pnl = last_trade.pnl(last_trade.exit_price.unwrap());
                    let pnl = last_trade.pnl;

                    self.events.on_trade_exit =
                        Some(StrategyOnTradeExitEvent { trade: *last_trade });

                    self.metrics.net_profit += pnl;
                    self.metrics.open_profit = 0.0;

                    if pnl > 0.0 {
                        self.metrics.gross_profit += pnl;
                        self.metrics.winning_trades += 1;
                    } else if pnl < 0.0 {
                        self.metrics.gross_loss += pnl.abs();
                        self.metrics.losing_trades += 1;
                    }

                    if last_trade.direction == TradeDirection::Long {
                        self.metrics.long_net_profit += pnl;
                    } else {
                        self.metrics.short_net_profit += pnl;
                    }

                    self.metrics.closed_trades += 1;
                }
            } else {
                create_new_trade = true;
            }

            if create_new_trade {
                let mut trade = Trade::new(unfilled_trade_direction);

                trade.fill_size = Some(1.0);

                if self.config.buy_with_equity {
                    let equity = self.config.initial_capital
                        + self.metrics.net_profit
                        + self.metrics.open_profit;

                    trade.fill_size = Some(fill_size(equity, orderbook_price.unwrap()));
                }

                trade.entry_price = orderbook_price;
                trade.entry_tick = Some(tick);

                self.trades.push(trade);
                self.events.on_trade_entry = Some(StrategyOnTradeEntryEvent { trade: trade });
            }

            self.unfilled_trade_direction = None;
        }

        if !self.config.on_bar_close {
            self.unfilled_trade_direction = direction;
        }

        if let Some(last_trade) = self.trades.last_mut() {
            if !last_trade.is_closed {
                self.metrics.open_profit = last_trade.pnl(close.unwrap());
            }
        }

        self.metrics.equity =
            self.config.initial_capital + self.metrics.net_profit + self.metrics.open_profit;
    }
}
