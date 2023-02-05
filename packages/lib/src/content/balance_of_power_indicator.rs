use crate::base::components::component_context::ComponentContext;

pub struct BalanceOfPowerIndicator {
    ctx: ComponentContext,
}

impl BalanceOfPowerIndicator {
    pub fn new(ctx: ComponentContext) -> Self {
        return BalanceOfPowerIndicator { ctx: ctx.clone() };
    }

    pub fn next(&mut self) -> Option<f64> {
        self.ctx.assert();

        let ctx = self.ctx.get();

        let close = ctx.close();
        let open = ctx.open();
        let high = ctx.high();
        let low = ctx.low();

        let value = match (close, open, high, low) {
            (Some(close), Some(open), Some(high), Some(low)) => {
                if high == low {
                    return None;
                }

                return Some((close - open) / (high - low));
            }
            _ => None,
        };

        return value;
    }
}
