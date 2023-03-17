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

pub struct MaxDrawdownMetricComponent {
    max_drawdown: f64,
    ctx: ComponentContext,
    strategy_ctx: StrategyComponentContext,
}

impl MaxDrawdownMetricComponent {
    pub fn new(ctx: ComponentContext, strategy_ctx: StrategyComponentContext) -> Self {
        return Self {
            ctx: ctx.clone(),
            strategy_ctx,
            max_drawdown: 0.0,
        };
    }

    pub fn next(&mut self, current: f64, highest: f64, lowest: f64) {
        self.ctx.on_next();

        // let dd = f64::max(
        //     self.highest_equity - self.bar_lowest_open_equity,
        //     self.metrics.max_drawdown,
        // );
    }
}
