use crate::{
    core::{context::Context, incremental::Incremental},
    strategy::{
        strategy::{Strategy, StrategyEvents, StrategyMetrics},
        trade::TradeDirection,
    },
};

pub struct EquityMetricsData {
    pub equity: f64,
    pub equity_min: f64,
    pub equity_max: f64,
    pub prev_equity: f64,
    /// Net current equity (initial capital + net profit)
    pub net_equity: f64,
    /// Lowest net equity value updated when trade is closed.
    pub net_equity_min: f64,
    /// Highest net equity value updated when trade is closed.
    pub net_equity_max: f64,
    /// Lowest open profit value for the current bar
    pub bar_open_profit_min: f64,
    /// Highest open profit value for the current bar
    pub bar_open_profit_max: f64,
    pub bar_equity_min: f64,
    pub bar_equity_max: f64,
}

impl EquityMetricsData {
    pub fn default(initial_capital: f64) -> Self {
        return Self {
            prev_equity: initial_capital,
            equity: initial_capital,
            equity_min: initial_capital,
            equity_max: initial_capital,
            net_equity: initial_capital,
            net_equity_min: initial_capital,
            net_equity_max: initial_capital,
            bar_open_profit_min: 0.0,
            bar_open_profit_max: 0.0,
            bar_equity_min: initial_capital,
            bar_equity_max: initial_capital,
        };
    }
}

/// Calculates basic equity metrics.
pub struct EquityMetrics {
    pub ctx: Context,
    pub data: EquityMetricsData,
}

impl EquityMetrics {
    pub fn new(ctx: Context, strategy: &Strategy) -> Self {
        return Self {
            ctx,
            data: EquityMetricsData::default(strategy.config.initial_capital),
        };
    }
}

impl Incremental<&Strategy, ()> for EquityMetrics {
    fn next(&mut self, strategy: &Strategy) {
        if let Some(e) = &strategy.events.on_trade_exit {
            self.data.net_equity = strategy.config.initial_capital + strategy.metrics.net_profit;
            self.data.net_equity_min = f64::min(self.data.net_equity_min, self.data.net_equity);
            self.data.net_equity_max = f64::max(self.data.net_equity_max, self.data.net_equity);
        }

        let last_trade = strategy.trades.last();

        if let Some(last_trade) = &last_trade {
            if !last_trade.is_closed {
                let mut lowest_price = self.ctx.bar.low();
                let mut highest_price = self.ctx.bar.high();

                if last_trade.direction == TradeDirection::Short {
                    let _lowest_price = lowest_price;

                    lowest_price = highest_price;
                    highest_price = _lowest_price;
                }

                self.data.bar_open_profit_min = last_trade.pnl(lowest_price);
                self.data.bar_open_profit_max = last_trade.pnl(highest_price);
            }
        }

        self.data.equity = strategy.metrics.equity;
        self.data.equity_min = f64::min(self.data.equity_min, self.data.equity);
        self.data.equity_max = f64::max(self.data.equity_max, self.data.equity);

        self.data.bar_equity_min = self.data.net_equity + self.data.bar_open_profit_min;
        self.data.bar_equity_max = self.data.net_equity + self.data.bar_open_profit_max;
    }
}
