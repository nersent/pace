use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

use crate::{
    common::src::ohlc4,
    core::{context::Context, incremental::Incremental},
};

use super::trade::{fill_size, StrategySignal, Trade, TradeDirection};

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
    pub initial_capital: f64,
    pub buy_with_equity: bool,
}

impl Default for StrategyConfig {
    fn default() -> Self {
        return Self {
            buy_with_equity: false,
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
    pub position_size: f64,
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
            position_size: 0.0,
        };
    }
}

/// Manages trades and provides data for all strategy components.
pub struct Strategy {
    pub ctx: Context,
    pub config: StrategyConfig,
    pub trades: Vec<Trade>,
    pub events: StrategyEvents,
    pub metrics: StrategyMetrics,
    unfilled_signal: StrategySignal,
    pub current_dir: Option<TradeDirection>,
}

impl Strategy {
    pub fn new(ctx: Context, config: StrategyConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            trades: Vec::new(),
            unfilled_signal: StrategySignal::Neutral,
            current_dir: None,
            events: StrategyEvents {
                on_trade_entry: None,
                on_trade_exit: None,
            },
            metrics: StrategyMetrics::default(config.initial_capital),
            config,
        };
    }
}

impl Incremental<StrategySignal, ()> for Strategy {
    fn next(&mut self, signal: StrategySignal) {
        let bar = &self.ctx.bar;
        let tick = bar.index();
        let open = bar.open();
        let close = bar.close();

        if self.config.on_bar_close {
            self.unfilled_signal = signal;
        }

        self.events.on_trade_entry = None;
        self.events.on_trade_exit = None;

        if self.unfilled_signal != StrategySignal::Neutral {
            let mut close_trade = false;
            let mut new_trade_dir: Option<TradeDirection> = None;

            let orderbook_price = if self.config.on_bar_close {
                close
            } else {
                open
            };

            if let Some(last_trade) = self.trades.last_mut() {
                let dir = last_trade.direction;

                if self.unfilled_signal == StrategySignal::Long {
                    close_trade = dir == TradeDirection::Short;
                    new_trade_dir = Some(TradeDirection::Long);
                }

                if self.unfilled_signal == StrategySignal::Short {
                    close_trade = dir == TradeDirection::Long;
                    new_trade_dir = Some(TradeDirection::Short);
                }

                if self.unfilled_signal == StrategySignal::LongEntry {
                    close_trade = dir == TradeDirection::Short;
                    new_trade_dir = Some(TradeDirection::Long);
                }

                if self.unfilled_signal == StrategySignal::ShortEntry {
                    close_trade = dir == TradeDirection::Long;
                    new_trade_dir = Some(TradeDirection::Short);
                }

                if self.unfilled_signal == StrategySignal::LongExit && dir == TradeDirection::Long {
                    close_trade = true;
                }

                if self.unfilled_signal == StrategySignal::ShortExit && dir == TradeDirection::Short
                {
                    close_trade = true;
                }

                if let Some(_new_trade_dir) = new_trade_dir {
                    let is_same_direction = !last_trade.is_closed && dir == _new_trade_dir;
                    close_trade = close_trade && !is_same_direction && !last_trade.is_closed;

                    if is_same_direction {
                        new_trade_dir = None;
                    }
                }

                if close_trade {
                    let exit_price = orderbook_price;

                    last_trade.exit_price = exit_price;
                    last_trade.exit_tick = Some(tick);
                    last_trade.is_closed = true;
                    last_trade.pnl = last_trade.pnl(last_trade.exit_price);
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

                    if dir == TradeDirection::Long {
                        self.metrics.long_net_profit += pnl;
                        self.metrics.position_size -= 1.0;
                    } else {
                        self.metrics.short_net_profit += pnl;
                        self.metrics.position_size += 1.0;
                    }

                    self.current_dir = None;
                    self.metrics.closed_trades += 1;
                }
            } else if !self.unfilled_signal.is_explicit_exit() {
                new_trade_dir = self.unfilled_signal.continous();
            }

            if let Some(new_trade_dir) = new_trade_dir {
                let entry_price = orderbook_price;

                let mut trade = Trade::new(new_trade_dir);

                trade.fill_size = 1.0;

                if self.config.buy_with_equity {
                    let equity = self.config.initial_capital
                        + self.metrics.net_profit
                        + self.metrics.open_profit;

                    trade.fill_size = fill_size(equity, entry_price);
                }

                trade.entry_price = entry_price;
                trade.entry_tick = Some(tick);

                self.trades.push(trade);
                self.events.on_trade_entry = Some(StrategyOnTradeEntryEvent { trade: trade });

                if trade.direction == TradeDirection::Long {
                    self.metrics.position_size += 1.0;
                    self.current_dir = Some(TradeDirection::Long);
                } else {
                    self.metrics.position_size -= 1.0;
                    self.current_dir = Some(TradeDirection::Short);
                }
            }

            self.unfilled_signal = StrategySignal::Neutral;
        }

        if !self.config.on_bar_close {
            self.unfilled_signal = signal;
        }

        if let Some(last_trade) = self.trades.last_mut() {
            if !last_trade.is_closed {
                self.metrics.open_profit = last_trade.pnl(close);
            }
        }

        self.metrics.equity =
            self.config.initial_capital + self.metrics.net_profit + self.metrics.open_profit;
    }
}
