use colored::Colorize;

use crate::base::{
    components::{
        common::{
            mean_component::MeanComponent,
            welfords_stdev_component::WelfordsStandardDeviationComponent,
        },
        component_context::ComponentContext,
        component_default::ComponentDefault,
    },
    strategy::trade::{
        compute_fill_size, compute_pnl, compute_return, compute_trade_pnl, Trade, TradeDirection,
    },
};
use pyo3::prelude::*;

#[derive(Debug, Clone, Copy)]
#[pyclass(name = "Equity")]
pub struct Equity {
    #[pyo3(get)]
    pub capital: f64,
    #[pyo3(get)]
    pub returns: f64,
    #[pyo3(get)]
    pub returns_mean: f64,
    #[pyo3(get)]
    pub returns_stdev: f64,
    #[pyo3(get)]
    pub trade_pnl: f64,
    #[pyo3(get)]
    pub trade_fill_size: Option<f64>,
    pub net_profit: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct EquityMetricConfig {
    pub initial_capital: f64,
}

impl ComponentDefault for EquityMetricConfig {
    fn default(ctx: ComponentContext) -> Self {
        return Self {
            initial_capital: 1000.0,
        };
    }
}

pub struct EquityMetric {
    pub config: EquityMetricConfig,
    ctx: ComponentContext,
    prev_capital: f64,
    capital_before_trade: f64,
    stdev: WelfordsStandardDeviationComponent,
    mean_returns: MeanComponent,
    trade_fill_size: Option<f64>,
    prev_trade_direction: Option<TradeDirection>,
    prev_trade: Option<Trade>,
    prev_trade_fill_price: Option<f64>,
    prev_trade_pnl: Option<f64>,
    net_profit: f64,
}

impl EquityMetric {
    pub fn new(ctx: ComponentContext, config: EquityMetricConfig) -> Self {
        return EquityMetric {
            ctx: ctx.clone(),
            prev_capital: config.initial_capital,
            capital_before_trade: config.initial_capital,
            stdev: WelfordsStandardDeviationComponent::new(ctx.clone()),
            mean_returns: MeanComponent::new(ctx.clone()),
            trade_fill_size: None,
            prev_trade_direction: None,
            prev_trade_fill_price: None,
            prev_trade_pnl: None,
            prev_trade: None,
            config,
            net_profit: 0.0,
        };
    }

    pub fn next(&mut self, trade: Option<&Trade>) -> Equity {
        self.ctx.assert();

        let ctx = self.ctx.get();
        let tick = ctx.current_tick;
        let price = ctx.close().unwrap();
        let open = ctx.open().unwrap();
        // !TODO
        // let is_up = ctx.is_up();
        // let price = if is_up {
        //     ctx.close().unwrap()
        // } else {
        //     ctx.open().unwrap()
        // };

        let price = ctx.close().unwrap();
        let price = ctx.close().unwrap();
        let mut current_capital = self.capital_before_trade;
        let mut trade_pnl = 0.0;

        if let Some(trade) = trade {
            let is_at_exit = trade.is_at_exit(tick)
                || self.prev_trade_direction.is_some()
                    && self.prev_trade_direction.unwrap() != trade.direction;
            /*|| self.prev_trade_direction.is_some()
            && self.prev_trade_direction.unwrap() != trade.direction;*/

            if is_at_exit {
                trade_pnl = compute_trade_pnl(
                    // self.trade_fill_size.unwrap(),
                    1.0,
                    trade.entry_price.unwrap(),
                    price,
                    trade.direction == TradeDirection::Long,
                );
                current_capital += trade_pnl;
                self.capital_before_trade = current_capital;
                self.prev_trade_direction = None;
                if self.prev_trade.is_some() {
                    self.net_profit += self.prev_trade.unwrap().exit_price.unwrap()
                        - self.prev_trade.unwrap().entry_price.unwrap();
                }
                self.prev_trade = Some(trade.clone());

                // println!("[STATUS] AT EXIT")
            }

            if trade.is_at_entry(tick) {
                self.trade_fill_size = Some(compute_fill_size(
                    current_capital,
                    trade.entry_price.unwrap(),
                ));
                // trade_pnl = compute_trade_pnl(
                //     0.0,
                //     trade.entry_price.unwrap(),
                //     price,
                //     trade.direction == TradeDirection::Long,
                // );
                // current_capital += trade_pnl;
                self.prev_trade_fill_price = trade.entry_price;
                // println!("[STATUS] AT ENTRY");
                self.prev_trade_direction = Some(trade.direction);
            }

            if !is_at_exit && trade.is_active() {
                // trade_pnl = trade
                //     .pnl(self.trade_fill_size.unwrap(), price)
                //     .unwrap_or(0.0);
                // trade_pnl = trade.pnl(1.0, price).unwrap();
                trade_pnl = compute_trade_pnl(
                    1.0,
                    trade.entry_price.unwrap(),
                    price,
                    trade.direction == TradeDirection::Long,
                );
                current_capital += trade_pnl;
            }
        }

        // println!("{} | {}", current_capital, self.prev_capital);

        let returns = compute_return(current_capital, self.prev_capital);
        // let returns = compute_return(trade_pnl, self.prev_trade_pnl.unwrap_or(0.0));
        let returns_mean = self.mean_returns.next(returns);
        let returns_stdev = self.stdev.next(returns);

        self.prev_capital = current_capital;
        self.prev_trade_pnl = Some(trade_pnl);

        let capital = self.config.initial_capital + self.net_profit + trade_pnl;

        return Equity {
            capital: current_capital,
            returns,
            returns_mean,
            returns_stdev,
            trade_pnl,
            trade_fill_size: self.trade_fill_size,
            net_profit: self.net_profit,
        };
    }
}
