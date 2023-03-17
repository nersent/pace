use std::{cmp::min, rc::Rc};

use crate::base::{
    components::{
        common::{
            mean_component::MeanComponent,
            welfords_stdev_component::WelfordsStandardDeviationComponent,
        },
        component_context::ComponentContext,
        component_default::ComponentDefault,
    },
    strategy::orderbook::Order,
};

use super::{
    metrics::{
        max_drawdown_metric::MaxDrawdownMetric,
        profit::{
            compute_avg_losing_trade, compute_avg_trade, compute_avg_win_loss_ratio,
            compute_avg_winning_trade, compute_long_net_profit_ratio,
            compute_percent_profitable_trades, compute_profit_factor,
        },
    },
    orderbook::{OrderBook, OrderBookConfig},
    trade_refactor::{compute_fill_size, Trade, TradeDirection},
};

pub struct StrategyExecutionContextConfig {
    /**
    Enables an additional calculation on bar close, allowing market orders to enter on the same tick the order is placed
    */
    pub on_bar_close: bool,
    pub continous: bool,
    pub initial_capital: f64,
    pub buy_with_equity: bool,
}
pub struct StrategyOnTradeEntryEvent {
    pub trade: Trade,
}

pub struct StrategyOnTradeExitEvent {
    pub trade: Trade,
    pub pnl: f64,
}

pub struct StrategyExecutionContextEvents {
    pub on_trade_entry: Option<StrategyOnTradeEntryEvent>,
    pub on_trade_exit: Option<StrategyOnTradeExitEvent>,
}

pub struct StrategyExecutionContext {
    ctx: ComponentContext,
    pub config: StrategyExecutionContextConfig,
    pub trades: Vec<Trade>,
    pub unfilled_trade_direction: Option<TradeDirection>,
    pub events: StrategyExecutionContextEvents,
    /// Current equity (initial capital + net profit + open profit). In TradingView `strategy.equity`
    pub equity: f64,
    /// Net current equity (initial capital + net profit)
    pub net_equity: f64,
    /// The overall profit or loss. In TradingView `strategy.netprofit`
    pub net_profit: f64,
    /// Current unrealized profit or loss for all open positions. In TradingView `strategy.openprofit`
    pub open_profit: f64,
}

impl StrategyExecutionContext {
    pub fn new(ctx: ComponentContext, config: StrategyExecutionContextConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            trades: Vec::new(),
            unfilled_trade_direction: None,
            events: StrategyExecutionContextEvents {
                on_trade_entry: None,
                on_trade_exit: None,
            },
            equity: config.initial_capital,
            net_equity: config.initial_capital,
            net_profit: 0.0,
            open_profit: 0.0,
            config,
        };
    }

    pub fn next(&mut self, direction: Option<TradeDirection>) {
        self.ctx.on_next();

        let ctx = self.ctx.get();
        let tick = ctx.current_tick;
        let open = ctx.open();
        let close = ctx.close();

        if self.config.on_bar_close {
            self.unfilled_trade_direction = direction;
        }

        self.events.on_trade_entry = None;
        self.events.on_trade_exit = None;

        if let Some(unfilled_trade_direction) = self.unfilled_trade_direction {
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

                if self.config.continous {
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

                    self.net_profit += trade_pnl;
                    self.open_profit = 0.0;
                    self.net_equity = self.config.initial_capital + self.net_profit;

                    self.events.on_trade_exit = Some(StrategyOnTradeExitEvent {
                        trade: *last_trade,
                        pnl: trade_pnl,
                    });
                }
            } else {
                create_new_trade = true;
            }

            if create_new_trade {
                let mut trade = Trade::new(unfilled_trade_direction);

                trade.fill_size = Some(1.0);

                if self.config.buy_with_equity {
                    let mut last_trade = self.trades.last_mut();
                    let mut equity = self.equity;

                    if let Some(last_trade) = &mut last_trade {
                        let open_profit = last_trade.pnl(close.unwrap());
                        equity = self.config.initial_capital + self.net_profit + self.open_profit;
                    }

                    trade.fill_size = Some(compute_fill_size(equity, orderbook_price.unwrap()));
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

        let mut last_trade = self.trades.last_mut();

        if let Some(last_trade) = &mut last_trade {
            if !last_trade.is_closed {
                self.open_profit = last_trade.pnl(close.unwrap());
            }
        }

        self.equity = self.config.initial_capital + self.net_profit + self.open_profit;
    }
}
