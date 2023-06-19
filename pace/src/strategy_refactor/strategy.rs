use std::collections::VecDeque;

use colored::Colorize;
use polars::export::arrow::types::simd::f16x32;

use crate::{
    core::{
        context::Context,
        incremental::{Incremental, IncrementalDefault},
    },
    utils::float::Float64Utils,
};

use super::{
    common::{Qty, Signal},
    trade::{Trade, TradeDirection},
    utils::order_size,
};

#[derive(Debug, Clone, Copy)]
pub struct StrategyConfig {
    pub initial_capital: f64,
    pub process_orders_on_close: bool,
}

impl Default for StrategyConfig {
    fn default() -> Self {
        Self {
            initial_capital: 1000.0,
            process_orders_on_close: false,
        }
    }
}

#[derive(Debug)]
struct Order {
    pub id: String,
    pub direction: TradeDirection,
    pub size: f64,
    pub bar_index: usize,
}

pub struct Strategy {
    pub ctx: Context,
    config: StrategyConfig,
    // The amount of initial capital set in the strategy properties
    pub initial_capital: f64,
    pub closed_trades: VecDeque<Trade>,
    pub open_trades: VecDeque<Trade>,
    /// Current equity (initial capital + net profit + open profit).
    /// Same as PineScript `strategy.equity`
    pub equity: f64,
    /// Current unrealized profit or loss for all open positions. Same as `strategy.openprofit`
    pub open_profit: f64,
    /// Same as `strategy.initial_capital + strategy.net_profit`
    pub net_equity: f64,
    /// The overall profit or loss. Same as PineScript `strategy.netprofit`.
    pub net_profit: f64,
    /// Total value of all completed winning trades. Same as PineScript `strategy.grossprofit`.
    pub gross_profit: f64,
    /// Total value of all completed losing trades. Same as PineScript `strategy.grossloss`.
    pub gross_loss: f64,
    /// Total number of winning trades. Same as PineScript `strategy.wintrades`.
    pub winning_trades: usize,
    /// Total number of losing trades. Same as PineScript `strategy.losstrades`.
    pub losing_trades: usize,
    /// Direction and size of the current market position. If the value is > 0, the market position is long. If the value is < 0, the market position is short. The absolute value is the number of contracts/shares/lots/units in trade (position size)
    pub position_size: f64,
    order_queue: VecDeque<Order>,
    debug: bool,
}

impl Strategy {
    pub fn new(ctx: Context, config: StrategyConfig) -> Self {
        Self {
            initial_capital: config.initial_capital,
            closed_trades: VecDeque::new(),
            open_trades: VecDeque::new(),
            equity: config.initial_capital,
            net_equity: config.initial_capital,
            net_profit: 0.0,
            open_profit: 0.0,
            gross_loss: 0.0,
            gross_profit: 0.0,
            losing_trades: 0,
            winning_trades: 0,
            position_size: 0.0,
            ctx,
            config,
            order_queue: VecDeque::new(),
            debug: false,
        }
    }

    pub fn get_total_trades(&self) -> usize {
        return self.winning_trades + self.losing_trades;
    }

    pub fn get_instrument_price(&self) -> f64 {
        return self.ctx.round_to_min_tick(self.ctx.bar.close());
    }

    fn get_orderbook_price(&self) -> f64 {
        let price = if self.config.process_orders_on_close {
            self.ctx.bar.close()
        } else {
            self.ctx.bar.open()
        };
        return self.ctx.round_to_min_tick(price);
    }

    pub fn compute_order_size_from_qty(&self, qty: Qty) -> f64 {
        let size = match qty {
            Qty::Default => order_size(1.0, self.equity, 1.0, self.get_instrument_price(), 1.0),
            Qty::EquityPct(pct) => {
                order_size(pct, self.equity, 1.0, self.get_instrument_price(), 1.0)
            }
            Qty::Contracts(qty) => qty,
            _ => panic!("Invalid type of size: {:?}", qty),
        };
        return self.ctx.round_contracts(size);
    }

    fn can_ignore_new_order(&self, order: &Order) -> bool {
        if let Some(sym_info) = self.ctx.data.get_sym_info() {
            if !sym_info.min_qty.is_nan() {
                return order.size < sym_info.min_qty;
            }
        }
        return false;
    }

    fn log(&self, ctx: &str, sub_ctx: &str, msg: String) {
        let closed_trades = self.closed_trades.clone();
        let open_trades = self.open_trades.clone();
        let trades = closed_trades.iter().chain(open_trades.iter());
        let open_trades_count = trades.clone().filter(|t| !t.closed).count();
        let closed_trades_count = trades.clone().filter(|t| t.closed).count();

        println!(
            "\n\n{} {} -> {}: Position: {}, Open Trades: {}, Closed Trades: {}\n{}",
            self.ctx.bar.index().to_string().on_cyan().black().bold(),
            ctx.to_string().on_green().black().bold(),
            sub_ctx.to_string().green().bold().underline(),
            self.position_size,
            open_trades_count,
            closed_trades_count,
            msg
        );
    }

    fn queue_order(&mut self, order: Order) {
        if self.debug {
            self.log(
                "QUEUE_ORDER",
                "Got order",
                format!("{:?}", self.order_queue),
            );
        }

        if self.can_ignore_new_order(&order) {
            if self.debug {
                self.log("QUEUE_ORDER", "Ignoring", format!("{:?}", order));
            }
            return;
        }

        if self.debug {
            self.log("QUEUE_ORDER", "Adding to queue", format!("{:?}", order));
        }

        self.order_queue.push_back(order);
    }

    fn update_net_equity(&mut self) {
        self.net_equity = self.initial_capital + self.net_profit;
    }

    fn set_position_size(&mut self, size: f64) {
        
        if self.debug {
            self.log("SET_POSITION_SIZE", "Setting to", format!("{}", size));

            self.log(
                "SET_POSITION_SIZE",
                "Before",
                format!("{}", self.position_size),
            );
        }

        self.position_size = self.ctx.round_contracts(size);

        if self.debug {
            self.log(
                "SET_POSITION_SIZE",
                "After",
                format!("{}", self.position_size),
            );
        }
    }

    fn update(&mut self) {
        self.update_net_equity();

        let price = self.get_instrument_price();

        let mut open_profit = 0.0;

        for open_trade in &mut self.open_trades {
            open_trade.update_profit(price);
            open_profit += open_trade.profit;
        }

        self.open_profit = open_profit;
        self.equity = self.net_equity + open_profit;
    }

    fn can_process_order(&self, order: &Order) -> bool {
        let index = self.ctx.bar.index();
        if self.config.process_orders_on_close {
            return index >= order.bar_index;
        }
        return index > order.bar_index;
    }

    fn open_trade(&mut self, direction: TradeDirection, contracts: f64, entry_id: String) {
        if let Some(sym_info) = self.ctx.data.get_sym_info() {
            if !sym_info.min_qty.is_nan() {
                assert!(
                    contracts >= sym_info.min_qty,
                    "Trying to open a trade with less than the minimum quantity allowed. {} < {}",
                    contracts,
                    sym_info.min_qty
                );
            }
        }

        if self.debug {
            self.log(
                "OPEN_TRADE",
                "Opening new trade",
                format!("{:?} | {} | {}", direction, contracts, &entry_id),
            );
        }

        let mut trade = Trade::new(direction, contracts);

        trade.entry(self.get_orderbook_price(), entry_id, self.ctx.bar.index());
        trade.entry_time = self.ctx.bar.time();

        self.set_position_size(self.position_size + trade.get_directional_size());

        if self.debug {
            self.open_trades.push_back(trade.clone());
            self.log("OPEN_TRADE", "Opened new trade", format!("{:?}", &trade));
        } else {
            self.open_trades.push_back(trade);
        }
    }

    fn close_trade(&mut self, mut trade: Trade, exit_id: String) {
        if self.debug {
            self.log(
                "CLOSE_TRADE",
                "Closing trade",
                format!("{} | {:?}", &exit_id, trade),
            );
        }

        trade.close(self.get_orderbook_price(), exit_id, self.ctx.bar.index());
        trade.exit_time = self.ctx.bar.time();

        self.net_profit += trade.profit;

        if trade.profit > 0.0 {
            self.winning_trades += 1;
            self.gross_profit += trade.profit;
        } else {
            self.losing_trades += 1;
            self.gross_loss += trade.profit.abs();
        }

        self.set_position_size(self.position_size - trade.get_directional_size());

        if self.debug {
            self.closed_trades.push_back(trade.clone());
            self.log(
                "CLOSE_TRADE",
                "Closed trade",
                format!("{:?}\n----\n{:?}", &trade, self.open_trades.back()),
            );
        } else {
            self.closed_trades.push_back(trade);
        }
    }

    fn process_order_queue(&mut self) {
        loop {
            // println!("[bar_index] {}", self.ctx.bar.index());
            {
                let order = self.order_queue.front();
                if order.is_none() || !self.can_process_order(&order.unwrap()) {
                    break;
                }
            }

            let order = self.order_queue.pop_front().unwrap();

            let mut size = self.ctx.round_contracts(order.size);

            if self.debug {
                self.log(
                    "PROCESS_ORDER_QUEUE",
                    "Processing order",
                    format!("{} | {:?}", size, order),
                );
            }

            let mut i = 0;

            loop {
                if !self.ctx.validate_contracts(size) {
                    if self.debug {
                        self.log(
                            "PROCESS_ORDER_QUEUE",
                            "Invalid contract size. Breaking loop.",
                            format!("{:?}", size),
                        );
                    }

                    break;
                }

                let open_trade = self.open_trades.get_mut(i);

                if open_trade.is_none() {
                    if self.debug {
                        self.log(
                            "PROCESS_ORDER_QUEUE",
                            "No more open trades",
                            format!("{:?}", size),
                        );
                    }

                    break;
                }

                let open_trade = open_trade.unwrap();

                assert!(!open_trade.closed, "Open trade should not be closed");

                if open_trade.direction != order.direction {
                    // let size_diff = self.ctx.round_contracts(open_trade.size - size);
                    let size_diff = self.ctx.round_contracts(open_trade.size - size);
                    let close_trade = size_diff <= 0.0;

                    if self.debug {
                        println!(
                            "\n\n[{}] PROCESS_ORDER_QUEUE -> Modifying trade size\nOpen Trade Size: {} | Rounded Open Trade Size: {} | Size: {} | Rounded Size: {} | Size Diff: {} | Rounded Size Diff: {} | \n{:?}",
                            self.ctx.bar.index(),
                            open_trade.size,
                            self.ctx.round_contracts(open_trade.size),
                            size,
                            self.ctx.round_contracts(size),
                            size_diff, 
                            self.ctx.round_contracts(size_diff),
                            open_trade
                        );
                    }

                    if close_trade {
                        let closed_trade = self.open_trades.remove(i).unwrap();
                        size = size_diff.abs();

                        self.close_trade(closed_trade, order.id.clone());

                        continue;
                    }

                    open_trade.set_size(size_diff);

                    let mut partially_closed_trade = open_trade.clone();
                    partially_closed_trade.set_size(self.ctx.round_contracts(size));
                    self.close_trade(partially_closed_trade, order.id.clone());

                    size = 0.0;
                }

                i += 1;
            }

            if self.ctx.validate_contracts(size) {
                self.open_trade(order.direction, size, order.id.clone());
            }
        }

        self.update();
    }

    pub fn next_bar(&mut self) {
        if !self.config.process_orders_on_close {
            self.process_order_queue();
        }
    }

    fn create_order(&self, signal: Signal) -> Option<Order> {
        let bar_index = self.ctx.bar.index();
        match signal {
            Signal::Hold => None,
            Signal::Order(order) => {
                let size = self.compute_order_size_from_qty(order.qty);
                Some(Order {
                    direction: order.direction,
                    bar_index,
                    id: order.id,
                    size,
                })
            }
            _ => panic!("Invalid signal {:?}", signal),
        }
    }

    pub fn signal(&mut self, signal: Signal) {
        if self.debug {
            self.log("SIGNAL", "Got signal", format!("{:?}", signal));
        }
        if let Some(order) = self.create_order(signal) {
            self.queue_order(order);
        }
    }
}

impl Incremental<(), ()> for Strategy {
    fn next(&mut self, _: ()) {
        if self.config.process_orders_on_close {
            self.process_order_queue();
        }
    }
}
