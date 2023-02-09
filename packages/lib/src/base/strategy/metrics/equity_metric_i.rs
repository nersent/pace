use colored::Colorize;

use crate::base::{
    components::{
        common::{
            mean_component::MeanComponent,
            welfords_stdev_component::WelfordsStandardDeviationComponent,
        },
        component_context::ComponentContext,
    },
    strategy::trade::{
        compute_fill_size, compute_pnl, compute_return, compute_trade_pnl, Trade, TradeDirection,
    },
};

#[derive(Debug, Clone, Copy)]
pub struct Equity {
    pub capital: f64,
    pub returns: f64,
    pub returns_mean: f64,
    pub returns_stdev: f64,
    pub trade_pnl: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct EquityMetricConfig {
    pub initial_capital: f64,
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
    prev_trade_fill_price: Option<f64>,
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
            config,
        };
    }

    pub fn next(&mut self, trade: Option<&Trade>) -> Equity {
        self.ctx.assert();

        let ctx = self.ctx.get();
        let tick = ctx.current_tick;
        let price = ctx.close().unwrap();

        let mut current_capital = self.capital_before_trade;
        let mut trade_pnl = 0.0;

        if let Some(trade) = trade {
            let is_at_exit = trade.is_at_exit(tick)
                || self.prev_trade_direction.is_some()
                    && self.prev_trade_direction.unwrap() != trade.direction;

            if is_at_exit {
                trade_pnl = compute_trade_pnl(
                    self.trade_fill_size.unwrap(),
                    self.prev_trade_fill_price.unwrap(),
                    price,
                    self.prev_trade_direction.unwrap() == TradeDirection::Long,
                );
                current_capital += trade_pnl;

                // println!("[STATUS] AT EXIT")
            }

            if trade.is_at_entry(tick) {
                self.trade_fill_size = Some(compute_fill_size(
                    current_capital,
                    trade.entry_price.unwrap(),
                ));
                self.capital_before_trade = current_capital;
                self.prev_trade_fill_price = Some(price);
                // println!("[STATUS] AT ENTRY")
            }

            if !is_at_exit && trade.is_active() {
                trade_pnl = trade
                    .pnl(self.trade_fill_size.unwrap(), price)
                    .unwrap_or(0.0);
                current_capital += trade_pnl;
            }

            self.prev_trade_direction = Some(trade.direction);
        }

        // println!("{} | {}", current_capital, self.prev_capital);

        let returns = compute_return(current_capital, self.prev_capital);
        let returns_mean = self.mean_returns.next(returns);
        let returns_stdev = self.stdev.next(returns);

        self.prev_capital = current_capital;

        return Equity {
            capital: current_capital,
            returns,
            returns_mean,
            returns_stdev,
            trade_pnl,
        };
    }
}
