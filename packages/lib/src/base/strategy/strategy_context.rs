use std::cmp::min;

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
    metrics::{max_drawdown_metric::MaxDrawdownMetric, profit::compute_profit_factor},
    orderbook::{OrderBook, OrderBookConfig},
    trade::{compute_fill_size, compute_return, Trade, TradeDirection},
};

pub struct StrategyContextConfig {
    /**
    Enables an additional calculation on bar close, allowing market orders to enter on the same tick the order is placed
    */
    pub on_bar_close: bool,
    pub continous: bool,
    pub initial_capital: f64,
    pub buy_with_equity: bool,
}

impl ComponentDefault for StrategyContextConfig {
    fn default(ctx: ComponentContext) -> Self {
        return StrategyContextConfig {
            on_bar_close: false,
            continous: true,
            initial_capital: 1000.0,
            buy_with_equity: false,
        };
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct StrategyMetrics {
    pub open_profit: f64,
    pub net_profit: f64,
    pub gross_profit: f64,
    pub gross_loss: f64,
    pub equity: f64,
    pub closed_trades: usize,
    pub winning_trades: usize,
    pub losing_trades: usize,
    pub max_drawdown: f64,
    pub max_drawdown_percent: f64,
    pub low_open_profit: f64,
    pub high_open_profit: f64,
    pub low_equity: f64,
    pub high_equity: f64,
    pub max_run_up: f64,
    pub max_run_up_percent: f64,
    pub net_profit_percent: f64,
    pub gross_profit_percent: f64,
    pub gross_loss_percent: f64,
    pub percent_profitable: f64,
    pub profit_factor: f64,
    pub avg_winning_trade: f64,
    pub avg_losing_trade: f64,
    pub ratio_avg_win_avg_loss: f64,
}

pub struct StrategyContext {
    pub config: StrategyContextConfig,
    pub trades: Vec<Trade>,
    ctx: ComponentContext,
    unfilled_trade_direction: Option<TradeDirection>,
    pub metrics: StrategyMetrics,
    trade_fill_size: f64,
    prev_equity: f64,
    pub on_close_trade: bool,
    equity_max_drawdown_metric: MaxDrawdownMetric,
    pub highest_equity: f64,
    pub lowest_equity: f64,
    lowest_pnl: f64,
    max_equity: f64,
    pub trade_max_equity: f64,
}

impl StrategyContext {
    pub fn new(ctx: ComponentContext, config: StrategyContextConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            trades: Vec::new(),
            unfilled_trade_direction: None,
            metrics: StrategyMetrics {
                equity: config.initial_capital,
                open_profit: 0.0,
                net_profit: 0.0,
                gross_loss: 0.0,
                gross_profit: 0.0,
                closed_trades: 0,
                losing_trades: 0,
                gross_loss_percent: 0.0,
                gross_profit_percent: 0.0,
                net_profit_percent: 0.0,
                winning_trades: 0,
                max_drawdown: 0.0,
                max_run_up_percent: 0.0,
                low_open_profit: 0.0,
                high_open_profit: 0.0,
                low_equity: config.initial_capital,
                high_equity: config.initial_capital,
                max_run_up: 0.0,
                max_drawdown_percent: 0.0,
                avg_losing_trade: 0.0,
                avg_winning_trade: 0.0,
                percent_profitable: 0.0,
                profit_factor: 0.0,
                ratio_avg_win_avg_loss: 0.0,
            },
            trade_fill_size: 1.0,
            prev_equity: config.initial_capital,
            on_close_trade: false,
            equity_max_drawdown_metric: MaxDrawdownMetric::new(ctx.clone(), config.initial_capital),
            max_equity: config.initial_capital,
            lowest_pnl: 0.0,
            lowest_equity: config.initial_capital,
            highest_equity: config.initial_capital,
            trade_max_equity: config.initial_capital,
            config,
        };
    }

    pub fn next(&mut self, direction: Option<TradeDirection>) {
        self.ctx.on_next();

        let ctx = self.ctx.get();
        let tick = ctx.current_tick;
        let open = ctx.open();
        let close = ctx.close();
        let high = ctx.high();
        let low = ctx.low();

        let lowest_price = low
            .unwrap()
            .min(close.unwrap())
            .min(open.unwrap())
            .min(high.unwrap());
        let highest_price = high
            .unwrap()
            .max(close.unwrap())
            .max(open.unwrap())
            .max(low.unwrap());

        if self.config.on_bar_close {
            self.unfilled_trade_direction = direction;
        }

        let mut close_trade = false;
        let mut create_new_trade = false;
        let mut calculate_open_profit = true;

        if let Some(unfilled_trade_direction) = self.unfilled_trade_direction {
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

                    if close_trade {
                        calculate_open_profit = false;
                    }
                }

                if close_trade {
                    last_trade.exit_price = orderbook_price;
                    last_trade.exit_tick = Some(tick);
                    last_trade.is_closed = true;

                    let trade_pnl =
                        last_trade.pnl(self.trade_fill_size, last_trade.exit_price.unwrap());

                    self.metrics.net_profit += trade_pnl;
                    self.metrics.open_profit = 0.0;
                    self.metrics.closed_trades += 1;

                    if trade_pnl < 0.0 {
                        self.metrics.gross_loss += trade_pnl.abs();
                        self.metrics.losing_trades += 1;
                    } else if trade_pnl > 0.0 {
                        self.metrics.gross_profit += trade_pnl;
                        self.metrics.winning_trades += 1;
                    }

                    let open_profit = last_trade.pnl(self.trade_fill_size, close.unwrap());

                    let equity = self.config.initial_capital + self.metrics.net_profit;

                    self.lowest_equity = f64::min(self.lowest_equity, equity);
                    self.highest_equity = f64::max(self.highest_equity, equity);
                    // println!("XDDD {}", tick);
                    // self.lowest_equity = self.lowest_equity.min(equity);
                }
            } else {
                create_new_trade = true;
            }

            if create_new_trade {
                if self.config.buy_with_equity {
                    let mut last_trade = self.trades.last_mut();
                    let prev_equity = self.metrics.equity;
                    let mut equity = self.metrics.equity;

                    if let Some(last_trade) = &mut last_trade {
                        let open_profit = last_trade.pnl(self.trade_fill_size, close.unwrap());
                        equity = self.config.initial_capital
                            + self.metrics.net_profit
                            + self.metrics.open_profit;
                    }

                    self.trade_fill_size = compute_fill_size(equity, orderbook_price.unwrap());
                }

                let mut trade = Trade::new(unfilled_trade_direction);
                trade.entry_price = orderbook_price;
                trade.entry_tick = Some(tick);
                self.trades.push(trade);
            }

            self.unfilled_trade_direction = None;
        }

        if !self.config.on_bar_close {
            self.unfilled_trade_direction = direction;
        }

        let mut last_trade = self.trades.last_mut();

        if let Some(last_trade) = &mut last_trade {
            if calculate_open_profit && !last_trade.is_closed {
                self.metrics.open_profit = last_trade.pnl(self.trade_fill_size, close.unwrap());

                let low_price = if last_trade.direction == TradeDirection::Long {
                    lowest_price
                } else {
                    highest_price
                };

                let high_price = if last_trade.direction == TradeDirection::Long {
                    highest_price
                } else {
                    lowest_price
                };

                self.metrics.low_open_profit = last_trade.pnl(self.trade_fill_size, low_price);
                self.metrics.high_open_profit = last_trade.pnl(self.trade_fill_size, high_price);
            }

            // let low_pnl = last_trade.pnl(self.trade_fill_size, low.unwrap());
            // let high_pnl = last_trade.pnl(self.trade_fill_size, high.unwrap());

            // let low_equity = self.config.initial_capital + self.metrics.net_profit + low_pnl;
            // let high_equity = self.config.initial_capital + self.metrics.net_profit + high_pnl;

            // self.highest_equity = self.highest_equity.max(high_equity);
            // self.lowest_equity = self.lowest_equity.min(low_equity);

            // let equity =
            //     self.config.initial_capital + self.metrics.net_profit + self.metrics.open_profit;

            // self.highest_equity = self.highest_equity.max(equity);

            // self.metrics.max_drawdown = self
            //     .metrics
            //     .max_drawdown
            //     .max(self.highest_equity - self.lowest_equity);

            // self.metrics.max_drawdown = self
            //     .metrics
            //     .max_drawdown
            //     .max(self.highest_equity - self.lowest_equity);

            // self.lowest_pnl = self.lowest_pnl.min(lowest_trade_pnl);
            // self.metrics.max_drawdown = -self.lowest_pnl;
        }

        self.metrics.equity =
            self.config.initial_capital + self.metrics.net_profit + self.metrics.open_profit;

        self.metrics.low_equity =
            self.config.initial_capital + self.metrics.net_profit + self.metrics.low_open_profit;

        self.metrics.high_equity =
            self.config.initial_capital + self.metrics.net_profit + self.metrics.high_open_profit;

        // if self.metrics.low_equity < self.lowest_equity {
        //     self.lowest_equity = self.metrics.low_equity;
        // }

        // if self.metrics.high_equity > self.highest_equity {
        //     self.highest_equity = self.metrics.high_equity;
        // }

        // self.metrics.max_runup = self.highest_equity - self.lowest_equity;
        self.metrics.max_run_up = f64::max(
            self.metrics.high_equity - self.lowest_equity,
            self.metrics.max_run_up,
        );

        self.metrics.max_drawdown = f64::max(
            self.highest_equity - self.metrics.low_equity,
            self.metrics.max_drawdown,
        );

        self.trade_max_equity = f64::max(self.trade_max_equity, self.metrics.high_equity);

        self.metrics.max_drawdown_percent = self.metrics.max_drawdown / self.highest_equity;

        self.metrics.max_run_up_percent = self.metrics.max_run_up / self.trade_max_equity;

        self.on_close_trade = close_trade;

        self.prev_equity = self.metrics.equity;

        // self.metrics.net_profit_percent = self.metrics.net_profit / self.config.initial_capital;
        // self.metrics.gross_profit_percent = self.metrics.gross_profit / self.config.initial_capital;
        // self.metrics.gross_loss_percent = self.metrics.gross_loss / self.config.initial_capital;
        // self.metrics.profit_factor =
        //     compute_profit_factor(self.metrics.gross_profit, self.metrics.gross_loss);
        // self.metrics.avg_losing_trade = self.metrics.gross_loss / self.metrics.losing_trades;
        // self.metrics.avg_winning_trade = self.metrics.gross_profit / self.metrics.winning_trades;
        // self.metrics.avg_trade = self.metrics.net_profit / self.metrics.closed_trades;

        // let equity = self.metrics.equity;

        // if equity > self.max_equity {
        //     self.max_equity = equity;
        // }

        // self.metrics.max_drawdown = self.equity_max_drawdown_metric.next(self.metrics.equity);
    }
}
