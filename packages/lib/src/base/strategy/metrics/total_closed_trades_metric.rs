use crate::base::{components::component_context::ComponentContext, strategy::trade::Trade};

pub struct TotalClosedTradesMetric {
    ctx: ComponentContext,
    sum: usize,
}

impl TotalClosedTradesMetric {
    pub fn new(ctx: ComponentContext) -> Self {
        return TotalClosedTradesMetric {
            ctx: ctx.clone(),
            sum: 0,
        };
    }

    pub fn next(&mut self, trade: Option<&Trade>) -> usize {
        self.ctx.assert();

        if let Some(trade) = trade {
            if trade.is_closed {
                self.sum += 1;
            }
        }

        return self.sum;
    }
}
