use colored::Colorize;

use crate::base::{
    components::{
        common::{
            mean_component::MeanComponent,
            welfords_stdev_component::WelfordsStandardDeviationComponent,
        },
        component_context::ComponentContext,
    },
    strategy::trade::{compute_fill_size, compute_pnl, compute_return, Trade},
};

#[derive(Debug, Clone, Copy)]
pub struct Equity {
    pub equity: f64,
    pub fixed_returns: f64,
    pub returns: f64,
    pub returns_mean: f64,
    pub returns_stdev: f64,
    pub pnl: f64,
    pub trade_pnl: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct EquityMetricConfig {
    pub initial_capital: f64,
}

pub struct EquityMetric {
    pub config: EquityMetricConfig,
    ctx: ComponentContext,
    pub current_equity: f64,
    prev_equity: Option<f64>,
    pub trade_fill_size: Option<f64>,
    stdev: WelfordsStandardDeviationComponent,
    mean_returns: MeanComponent,
}

impl EquityMetric {
    pub fn new(ctx: ComponentContext, config: EquityMetricConfig) -> Self {
        return EquityMetric {
            ctx: ctx.clone(),
            config,
            current_equity: config.initial_capital,
            trade_fill_size: None,
            prev_equity: None,
            stdev: WelfordsStandardDeviationComponent::new(ctx.clone()),
            mean_returns: MeanComponent::new(ctx.clone()),
        };
    }

    pub fn next(&mut self, trade: Option<&Trade>) -> Equity {
        self.ctx.assert();

        let ctx = self.ctx.get();
        let tick = ctx.current_tick;
        let current_price = ctx.close().unwrap();
        let mut equity = self.current_equity;
        let mut _pnl = 0.0;

        if let Some(trade) = trade {
            if self.trade_fill_size.is_none()
                && !trade.is_closed
                && trade.entry_tick.is_some()
                && trade.exit_tick.is_none()
            {
                self.trade_fill_size = Some(compute_fill_size(
                    self.current_equity,
                    trade.entry_price.unwrap(),
                ));
            }

            if let Some(trade_fill_size) = self.trade_fill_size {
                _pnl = trade.pnl(trade_fill_size, current_price).unwrap_or(0.0);
                equity += _pnl;
            }
        }

        // computed from equity before trade was opened and current equity (trade pnL)
        let returns = compute_return(equity, self.current_equity);
        // computed from previous equity and current equity (trade pnL)
        let fixed_returns = compute_return(
            equity,
            self.prev_equity.unwrap_or(self.config.initial_capital),
        );
        let returns = returns;
        let mean_returns = self.mean_returns.next(returns);
        let stdev_returns = self.stdev.next(returns);
        let pnl = self
            .prev_equity
            .map(|prev_equity| compute_pnl(equity, self.current_equity))
            .unwrap_or(0.0);

        if let Some(trade) = trade {
            if trade.is_closed {
                self.trade_fill_size = None;
                self.current_equity = equity;
            }
        }

        self.prev_equity = Some(equity);

        return Equity {
            equity,
            returns,
            returns_mean: mean_returns,
            returns_stdev: stdev_returns,
            pnl,
            trade_pnl: _pnl,
            fixed_returns,
        };
    }
}
