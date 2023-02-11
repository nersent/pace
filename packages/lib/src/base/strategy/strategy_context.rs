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

pub struct StrategyMetrics {
    pub initial_capital: f64,
    pub open_profit: f64,
    pub net_profit: f64,
    pub equity: f64,
    pub returns: f64,
    pub returns_mean: f64,
    pub returns_stdev: f64,
}

pub struct StrategyContext {
    pub config: StrategyContextConfig,
    pub trades: Vec<Trade>,
    ctx: ComponentContext,
    unfilled_trade_direction: Option<TradeDirection>,
    pub metrics: StrategyMetrics,
    returns_stdev: WelfordsStandardDeviationComponent,
    returns_mean: MeanComponent,
    trade_fill_size: f64,
    prev_equity: f64,
    prev_open_profit: f64,
    last_negative_equity_tick: Option<usize>,
}

impl StrategyContext {
    pub fn new(ctx: ComponentContext, config: StrategyContextConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            trades: Vec::new(),
            unfilled_trade_direction: None,
            metrics: StrategyMetrics {
                initial_capital: config.initial_capital,
                open_profit: 0.0,
                net_profit: 0.0,
                equity: config.initial_capital,
                returns: 0.0,
                returns_mean: 0.0,
                returns_stdev: 0.0,
            },
            returns_stdev: WelfordsStandardDeviationComponent::new(ctx.clone()),
            returns_mean: MeanComponent::new(ctx.clone()),
            trade_fill_size: 1.0,
            prev_open_profit: 0.0,
            prev_equity: config.initial_capital,
            config,
            last_negative_equity_tick: None,
        };
    }

    pub fn next(&mut self, direction: Option<TradeDirection>) {
        self.ctx.on_next();

        let ctx = self.ctx.get();
        let tick = ctx.current_tick;
        let close = ctx.close();
        let open = ctx.open();

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
                        equity = self.metrics.initial_capital
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
            }
        }

        self.metrics.equity =
            self.metrics.initial_capital + self.metrics.net_profit + self.metrics.open_profit;

        self.metrics.returns = compute_return(self.metrics.equity, self.prev_equity);
        self.metrics.returns_mean = self.returns_mean.next(self.metrics.returns);
        self.metrics.returns_stdev = self.returns_stdev.next(self.metrics.returns);
        // if let Some(last_trade) = &mut last_trade {
        //     if calculate_open_profit && !last_trade.is_closed {
        //         self.metrics.returns =
        //             compute_return(self.metrics.net_profit, self.prev_open_profit);
        //     }
        // } else {
        //     self.metrics.returns = 0.0;
        // }

        // if self.metrics.equity < 0.0 {
        //     self.last_negative_equity_tick = Some(tick);
        // }

        // let can_compute_returns = self
        //     .last_negative_equity_tick
        //     .map(|x| tick - x > 2)
        //     .unwrap_or(true);

        // self.metrics.returns = if can_compute_returns {
        //     compute_return(self.metrics.equity, self.prev_equity)
        // } else {
        //     0.0
        // };

        self.prev_equity = self.metrics.equity;
        self.prev_open_profit = self.metrics.net_profit;
    }
}
