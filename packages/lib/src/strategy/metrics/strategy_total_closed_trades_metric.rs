use crate::{
    components::component_context::ComponentContext,
    strategy::{
        strategy_utils::{compute_omega_ratio, compute_sharpe_ratio},
        trade::Trade,
    },
};

use super::strategy_equity_metric::StrategyEquity;

pub struct StrategyTotalClosedTradesMetric {
    ctx: ComponentContext,
    sum: usize,
}

impl StrategyTotalClosedTradesMetric {
    pub fn new(ctx: ComponentContext) -> Self {
        return StrategyTotalClosedTradesMetric {
            ctx: ctx.clone(),
            sum: 0,
        };
    }

    pub fn next(&mut self, trade: Option<Trade>) -> usize {
        self.ctx.assert();

        if let Some(trade) = trade {
            if trade.is_closed {
                self.sum += 1;
            }
        }

        return self.sum;
    }
}
