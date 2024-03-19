use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

use crate::{
    common::src::ohlc4,
    core::{context::Context, incremental::Incremental},
    utils::float::Float64Utils,
};

use super::trade::{fill_size, trade_pnl, StrategySignal, Trade, TradeDirection};

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
    pub prev_equity: f64,
}

impl Strategy {
    pub fn new(ctx: Context, config: StrategyConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            trades: Vec::new(),
            unfilled_signal: StrategySignal::Hold,
            current_dir: None,
            events: StrategyEvents {
                on_trade_entry: None,
                on_trade_exit: None,
            },
            metrics: StrategyMetrics::default(config.initial_capital),
            prev_equity: config.initial_capital,
            config,
        };
    }

    fn process_orderbook(&mut self) {
        let bar = &self.ctx.bar;
        let tick = bar.index();
        let open = bar.open();
        let close = bar.close();

        self.events.on_trade_entry = None;
        self.events.on_trade_exit = None;

        if self.unfilled_signal != StrategySignal::Hold {
            let mut close_trade = false;
            let mut new_trade_dir: Option<TradeDirection> = None;
            let mut new_trade_qty = f64::NAN;

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

                if self.unfilled_signal == StrategySignal::LongExit && dir == TradeDirection::Long
                    || self.unfilled_signal == StrategySignal::Exit
                {
                    close_trade = !last_trade.is_closed;
                }

                if self.unfilled_signal == StrategySignal::ShortExit && dir == TradeDirection::Short
                    || self.unfilled_signal == StrategySignal::Exit
                {
                    close_trade = !last_trade.is_closed;
                }

                if let StrategySignal::Sized(qty) = self.unfilled_signal {
                    if qty.is_zero() {
                        close_trade = !last_trade.is_closed;
                    } else {
                        let dir = if qty > 0.0 {
                            TradeDirection::Long
                        } else {
                            TradeDirection::Short
                        };
                        close_trade = dir != last_trade.direction;
                        new_trade_dir = Some(dir);
                        new_trade_qty = qty.abs();
                    }
                }

                if let StrategySignal::Dynamic(qty) = self.unfilled_signal {
                    let dir = if qty > 0.0 {
                        TradeDirection::Long
                    } else {
                        TradeDirection::Short
                    };

                    close_trade =
                        (dir != last_trade.direction || qty.is_zero()) && !last_trade.is_closed;

                    if !close_trade {}

                    // if qty.is_zero() {
                    //     close_trade = !last_trade.is_closed;
                    // } else {
                    //     let dir = if qty > 0.0 {
                    //         TradeDirection::Long
                    //     } else {
                    //         TradeDirection::Short
                    //     };

                    //     if dir != last_trade.direction {
                    //         close_trade = dir != last_trade.direction;
                    //         new_trade_dir = Some(dir);
                    //         new_trade_qty = qty.abs();
                    //     } else {
                    //         // Decrease or increase equity exposure by filling contracts differece, not by closing trade
                    //         let size_diff = last_trade.size.abs() - qty.abs();

                    //         if size_diff.is_zero() {
                    //         } else {
                    //             let total_equity = self.config.initial_capital
                    //                 + self.metrics.net_profit
                    //                 + last_trade.pnl(orderbook_price);

                    //             let frozen_equity = self.config.initial_capital
                    //                 + self.metrics.net_profit
                    //                 - last_trade.fill_size * last_trade.entry_price;

                    //             let equity_in_trade = total_equity - frozen_equity;

                    //             let adjusted_total_equity = total_equity * qty.abs();

                    //             let capital_to_adjust = equity_in_trade - adjusted_total_equity;

                    //             let adjust_contract_size =
                    //                 fill_size(capital_to_adjust, orderbook_price);

                    //             self.metrics.net_profit += capital_to_adjust;

                    //             last_trade.fill_size = last_trade.fill_size - adjust_contract_size;

                    //             // let equity_in_trade = last_trade.fill_size * last_trade.entry_price;
                    //             // let equity_in_trade_percentage = last_trade.size;

                    //             // let trade_contracts = last_trade.fill_size;

                    //             // if dir == TradeDirection::Long {
                    //             //     if size_diff > 0.0 {
                    //             //         let equity_to_sell = equity_in_trade
                    //             //             * (equity_in_trade_percentage - qty.abs());

                    //             //         let sell_contract_size =
                    //             //             fill_size(equity_to_sell, orderbook_price);

                    //             //         last_trade.fill_size =
                    //             //             last_trade.fill_size - sell_contract_size;
                    //             //     }
                    //             // }

                    //             // let equity_in_trade = last_trade.fill_size * last_trade.entry_price;
                    //             // let equity_in_trade = last_trade.fill_size * last_trade.entry_price;

                    //             // let equity_not_in_trade = self.config.initial_capital
                    //             //     + self.metrics.net_profit
                    //             //     - equity_in_trade;
                    //             // // let equity_not_in_trade_percentage =
                    //             // //     1.0 - equity_in_trade_percentage;

                    //             // let pnl = last_trade.pnl(orderbook_price);

                    //             // if dir == TradeDirection::Long {
                    //             //     if size_diff < 0.0 {
                    //             //         let equity_to_sell = equity_in_trade
                    //             //             * (equity_in_trade_percentage - qty.abs());

                    //             //         let equity_to_sell_fill_size =
                    //             //             fill_size(equity_to_sell, orderbook_price);

                    //             //         last_trade.fill_size =
                    //             //             last_trade.fill_size - equity_to_sell_fill_size;

                    //             //         self.metrics.net_profit += . - pnl;
                    //             //     }
                    //             // }
                    //         }
                    //     }
                    // }
                }

                if let Some(_new_trade_dir) = new_trade_dir {
                    let is_same_direction = !last_trade.is_closed && dir == _new_trade_dir;
                    close_trade = close_trade && !is_same_direction && !last_trade.is_closed;

                    if is_same_direction {
                        if let StrategySignal::Sized(qty) = self.unfilled_signal {
                            if !last_trade.is_closed {
                                if last_trade.size.abs().compare(new_trade_qty) {
                                    new_trade_dir = None;
                                    new_trade_qty = f64::NAN;
                                } else {
                                    close_trade = true;
                                }
                            }
                        } else {
                            new_trade_dir = None;
                            new_trade_qty = f64::NAN;
                        }
                    } else {
                        if new_trade_qty.is_nan() {
                            new_trade_qty = 1.0;
                        }
                    }
                }

                if close_trade {
                    if last_trade.is_closed {
                        panic!("Trying to close already closed trade");
                    }

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
            } else {
                if self.unfilled_signal == StrategySignal::Long
                    || self.unfilled_signal == StrategySignal::LongEntry
                {
                    new_trade_dir = Some(TradeDirection::Long);
                    new_trade_qty = 1.0;
                }
                if self.unfilled_signal == StrategySignal::Short
                    || self.unfilled_signal == StrategySignal::ShortEntry
                {
                    new_trade_dir = Some(TradeDirection::Short);
                    new_trade_qty = 1.0;
                }
                if let StrategySignal::Sized(qty) = self.unfilled_signal {
                    if !qty.is_zero() {
                        new_trade_dir = Some(if qty > 0.0 {
                            TradeDirection::Long
                        } else {
                            TradeDirection::Short
                        });
                        new_trade_qty = qty.abs();
                    }
                    // println!("{:?}", self.unfilled_signal);
                }
                if let StrategySignal::Dynamic(qty) = self.unfilled_signal {
                    if !qty.is_zero() {
                        new_trade_dir = Some(if qty > 0.0 {
                            TradeDirection::Long
                        } else {
                            TradeDirection::Short
                        });
                        new_trade_qty = qty.abs();
                    }
                    // println!("{:?}", self.unfilled_signal);
                }
                // if !self.unfilled_signal.is_explicit_exit() {
                //     new_trade_dir = self.unfilled_signal.continous();
                // }
            }

            if let Some(new_trade_dir) = new_trade_dir {
                let entry_price = orderbook_price;

                let mut trade = Trade::new(new_trade_dir);

                if new_trade_qty.is_nan() {
                    panic!("Trade qty is NaN");
                }

                if new_trade_qty.is_zero() {
                    panic!("Trade qty is zero");
                }

                trade.fill_size = new_trade_qty;

                if self.config.buy_with_equity {
                    let equity = self.config.initial_capital + self.metrics.net_profit;
                    //  + self.metrics.open_profit;

                    let equity = equity * new_trade_qty;

                    trade.fill_size = fill_size(equity, entry_price);
                } else {
                    trade.fill_size = fill_size(1000.0, entry_price);
                }

                trade.entry_price = entry_price;
                trade.entry_tick = Some(tick);
                trade.size = new_trade_qty;

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

            self.unfilled_signal = StrategySignal::Hold;
        }

        if let Some(last_trade) = self.trades.last_mut() {
            if !last_trade.is_closed {
                self.metrics.open_profit = last_trade.pnl(close);
            }
        }

        self.prev_equity = self.metrics.equity;

        self.metrics.equity =
            self.config.initial_capital + self.metrics.net_profit + self.metrics.open_profit;
    }

    pub fn next_bar(&mut self) {
        if !self.config.on_bar_close {
            self.process_orderbook();
        }
    }

    pub fn cancel_orders(&mut self) {
        self.events.on_trade_entry = None;
        self.events.on_trade_exit = None;
        self.unfilled_signal = StrategySignal::Hold;
    }
}

impl Incremental<StrategySignal, ()> for Strategy {
    fn next(&mut self, signal: StrategySignal) {
        if self.config.on_bar_close {
            self.unfilled_signal = signal;
            self.process_orderbook();
        } else {
            self.unfilled_signal = signal;
        }
    }
}
