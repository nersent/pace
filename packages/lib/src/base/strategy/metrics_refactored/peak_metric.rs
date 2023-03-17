use crate::base::{
    components::{component_context::ComponentContext, component_default::ComponentDefault},
    strategy::{
        strategy_component_context::StrategyComponentContext,
        strategy_execution_context::StrategyExecutionContext, trade_refactor::TradeDirection,
    },
};

use super::utils::{
    compute_avg_losing_trade, compute_avg_trade, compute_avg_win_loss_ratio,
    compute_avg_winning_trade, compute_long_net_profit_ratio, compute_percent_profitable_trades,
    compute_profit_factor,
};

#[derive(Debug, Clone, Copy)]
pub struct PeakMetric {
    pub intra_trade_max_drawdown_percent: f64,
    // Maximum equity drawdown value for the whole trading interval. In TradingView `strategy.max_drawdown`
    pub equity_max_drawdown: f64,
    pub equity_max_drawdown_percent: f64,
}

impl Default for PeakMetric {
    fn default() -> Self {
        return Self {
            intra_trade_max_drawdown_percent: 0.0,
            equity_max_drawdown: 0.0,
            equity_max_drawdown_percent: 0.0,
        };
    }
}

pub struct PeakMetricComponent {
    pub res: PeakMetric,
    ctx: ComponentContext,
    strategy_ctx: StrategyComponentContext,
    current_trade_highest_equity: f64,
    current_trade_max_drawdown: f64,
    bar_lowest_open_equity: f64,
    bar_highest_open_equity: f64,
    bar_lowest_open_profit: f64,
    bar_highest_open_profit: f64,
    highest_equity: f64,
    lowest_equity: f64,
}

impl PeakMetricComponent {
    pub fn new(ctx: ComponentContext, strategy_ctx: StrategyComponentContext) -> Self {
        let initial_capital = strategy_ctx.get().config.initial_capital;
        return Self {
            res: PeakMetric::default(),
            ctx: ctx.clone(),
            strategy_ctx,
            current_trade_highest_equity: initial_capital,
            bar_lowest_open_equity: initial_capital,
            bar_highest_open_equity: initial_capital,
            bar_lowest_open_profit: 0.0,
            bar_highest_open_profit: 0.0,
            current_trade_max_drawdown: 0.0,
            highest_equity: initial_capital,
            lowest_equity: initial_capital,
        };
    }

    pub fn next(&mut self) {
        self.ctx.on_next();

        let ctx = self.ctx.get();
        let s = self.strategy_ctx.get();
        let initial_capital = s.config.initial_capital;

        if let Some(e) = &s.events.on_trade_exit {
            let intra_trade_max_drawdown_percent =
                self.current_trade_max_drawdown / e.trade.entry_price.unwrap();

            self.res.intra_trade_max_drawdown_percent = f64::max(
                intra_trade_max_drawdown_percent,
                self.res.intra_trade_max_drawdown_percent,
            );

            self.lowest_equity = f64::min(self.lowest_equity, s.net_equity);
            self.highest_equity = f64::max(self.highest_equity, s.net_equity);

            self.current_trade_max_drawdown = 0.0;
            self.current_trade_highest_equity = s.net_equity;
        }

        let last_trade = s.trades.last();

        if let Some(last_trade) = &last_trade {
            if !last_trade.is_closed {
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

                self.bar_lowest_open_profit = last_trade.pnl(low_price);
                self.bar_highest_open_profit = last_trade.pnl(high_price);
            }
        }

        self.bar_lowest_open_equity = s.net_equity + self.bar_lowest_open_profit;

        self.bar_highest_open_equity = s.net_equity + self.bar_highest_open_profit;

        self.current_trade_max_drawdown = f64::max(
            self.current_trade_highest_equity - self.bar_lowest_open_equity,
            self.current_trade_max_drawdown,
        );

        self.res.equity_max_drawdown = f64::max(
            self.highest_equity - self.bar_lowest_open_equity,
            self.res.equity_max_drawdown,
        );
        self.res.equity_max_drawdown_percent = self.res.equity_max_drawdown / self.highest_equity;
    }
}
