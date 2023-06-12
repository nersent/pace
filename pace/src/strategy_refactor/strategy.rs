use std::collections::VecDeque;

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

pub struct StrategyConfig {
    pub initial_capital: f64,
    pub process_orders_on_close: bool,
    pub price_precision: usize,
    pub minimum_qty: f64,
}

impl IncrementalDefault for StrategyConfig {
    fn default(ctx: Context) -> Self {
        Self {
            initial_capital: 1000.0,
            process_orders_on_close: false,
            price_precision: 2, // 2 decimal places
            // https://www.tradingcode.net/tradingview/equity-percent-default-order/#order-size-formula
            minimum_qty: 0.000001,
        }
    }
}

#[derive(Debug)]
struct SignalQueueItem {
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
    queue: VecDeque<SignalQueueItem>,
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
            queue: VecDeque::new(),
        }
    }

    fn update_net_equity(&mut self) {
        self.net_equity = self.initial_capital + self.net_profit;
    }

    fn update(&mut self) {
        let price = self.get_instrument_price();

        let mut open_profit = 0.0;

        for open_trade in &mut self.open_trades {
            open_trade.update_profit(price);
            open_profit += open_trade.profit;
        }

        self.open_profit = open_profit;
        self.equity = self.net_equity + open_profit;
    }

    fn can_process_queue_item(&self, queue_item: &SignalQueueItem) -> bool {
        if self.config.process_orders_on_close {
            self.ctx.bar.index() == queue_item.bar_index
        } else {
            self.ctx.bar.index() > queue_item.bar_index
        }
    }

    fn process_queue(&mut self) {
        let bar_index = self.ctx.bar.index();

        // self.update();

        loop {
            if self.queue.front().is_none()
                || !self.can_process_queue_item(&self.queue.front().unwrap())
            {
                break;
            }

            let item = self.queue.pop_front().unwrap();

            let current_position = self.position_size;
            let bar_index = self.ctx.bar.index();
            let orderbook_fill_price = self.get_order_book_fill_price();
            // let orderbook_fill_price = self.ctx.bar.open();

            if current_position.is_zero() {
                let mut trade = Trade::new(item.direction, item.size);

                trade.entry(orderbook_fill_price, item.id, bar_index);

                self.position_size += trade.get_directional_size();
                self.open_trades.push_back(trade);
            } else {
                let trade = self.open_trades.front_mut();

                assert!(trade.is_some(), "Trade should be open");

                let mut trade = trade.unwrap();

                // @TODO: Support pyramiding
                let should_close_trade = self.position_size < 0.0
                    && item.direction == TradeDirection::Long
                    || self.position_size > 0.0 && item.direction == TradeDirection::Short;

                println!("\n\n{} {:?}", should_close_trade, item);

                if should_close_trade {
                    trade.close(orderbook_fill_price, item.id.clone(), bar_index);

                    self.net_profit += trade.profit;

                    if trade.profit > 0.0 {
                        self.winning_trades += 1;
                        self.gross_profit += trade.profit;
                    } else {
                        self.losing_trades += 1;
                        self.gross_loss += trade.profit.abs();
                    }

                    // self.update_net_equity();
                    self.net_equity = self.initial_capital + self.net_profit;

                    self.position_size -= trade.get_directional_size();
                    self.closed_trades
                        .push_back(self.open_trades.pop_front().unwrap());

                    let mut new_trade = Trade::new(item.direction, item.size);

                    new_trade.entry(orderbook_fill_price, item.id.clone(), bar_index);

                    self.position_size += new_trade.get_directional_size();
                    self.open_trades.push_back(new_trade);
                }
            }
        }

        self.update();
    }

    pub fn next_bar(&mut self) {
        if !self.config.process_orders_on_close {
            self.process_queue();
        }
    }

    fn get_instrument_price(&self) -> f64 {
        // @TODO: Cache and round given the precision
        return self.ctx.bar.close().round();
    }

    fn get_order_book_fill_price(&self) -> f64 {
        if self.config.process_orders_on_close {
            return self.ctx.bar.close().round();
        }
        // return self.ctx.bar.open().floor();
        return self.ctx.bar.open().round();
    }

    fn create_queue_item(&self, signal: Signal) -> SignalQueueItem {
        let options = signal.as_signal_options().unwrap();

        let size = match options.qty {
            Qty::Default => order_size(1.0, self.equity, 1.0, self.get_instrument_price(), 1.0),
            Qty::EquityPct(pct) => {
                order_size(pct, self.equity, 1.0, self.get_instrument_price(), 1.0)
            }
            Qty::Contracts(qty) => qty,
            _ => panic!("Invalid type of size: {:?}", options.qty),
            // Qty::Cash(
        };

        if size.is_nan() || size.is_zero() {
            panic!("Invalid size: {}", size);
        }

        // round size to 0.000001
        let size = (size * 1000000.0).round() / 1000000.0;

        return SignalQueueItem {
            id: options.id.unwrap(),
            direction: options.direction,
            size,
            bar_index: self.ctx.bar.index(),
        };
    }

    pub fn signal(&mut self, signal: Signal) {
        self.queue.push_back(self.create_queue_item(signal));
    }
}

impl Incremental<(), ()> for Strategy {
    fn next(&mut self, _: ()) {
        if self.config.process_orders_on_close {
            self.process_queue();
        }
    }
}
