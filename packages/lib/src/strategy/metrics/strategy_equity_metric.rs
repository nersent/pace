use colored::Colorize;

use crate::{
    components::{
        component_context::ComponentContext, mean::mean_component::MeanComponent,
        stdev::stdev_component::StandardDeviationComponent,
    },
    strategy::{
        action::StrategyActionKind,
        strategy_utils::{compute_fill_size, compute_pnl, compute_return, compute_sharpe_ratio},
        trade::{Trade, TradeDirection},
    },
};

#[derive(Debug, Clone, Copy)]
pub struct StrategyEquity {
    pub equity: f64,
    pub fixed_returns: f64,
    pub returns: f64,
    pub returns_mean: f64,
    pub returns_stdev: f64,
    pub pnl: f64,
    pub trade_pnl: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct StrategyEquityMetricConfig {
    pub initial_capital: f64,
}

pub struct StrategyEquityMetric {
    pub config: StrategyEquityMetricConfig,
    ctx: ComponentContext,
    pub current_equity: f64,
    prev_equity: Option<f64>,
    pub trade_fill_size: Option<f64>,
    stdev: StandardDeviationComponent,
    mean_returns: MeanComponent,
}

impl StrategyEquityMetric {
    pub fn new(ctx: ComponentContext, config: StrategyEquityMetricConfig) -> Self {
        return StrategyEquityMetric {
            ctx: ctx.clone(),
            config,
            current_equity: config.initial_capital,
            trade_fill_size: None,
            prev_equity: None,
            stdev: StandardDeviationComponent::new(ctx.clone()),
            mean_returns: MeanComponent::new(ctx.clone()),
        };
    }

    pub fn next(&mut self, trade: Option<Trade>) -> StrategyEquity {
        self.ctx.assert();

        let ctx = self.ctx.get();
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

            if trade.is_closed {
                self.trade_fill_size = None;
                self.current_equity = equity;
            }
        }

        // let returns = self
        //     .prev_equity
        //     .map(|prev_equity| compute_return(equity, prev_equity))
        //     .unwrap_or(0.0);

        let fixed_returns = compute_return(equity, self.current_equity);
        let returns = fixed_returns;
        let mean_returns = self.mean_returns.next(returns);
        let stdev_returns = self.stdev.next(returns);
        let pnl = self
            .prev_equity
            .map(|prev_equity| compute_pnl(equity, self.current_equity))
            .unwrap_or(0.0);

        self.prev_equity = Some(equity);

        return StrategyEquity {
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
