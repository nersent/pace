use crate::{components::component_context::ComponentContext, ta::bars::utils::BarUtils};
pub struct AroonIndicatorConfig {
    pub length: usize,
}

pub struct AroonIndicator {
    config: AroonIndicatorConfig,
    ctx: ComponentContext,
}

pub struct AroonIndicatorResult {
    pub up: Option<f64>,
    pub down: Option<f64>,
}

impl AroonIndicator {
    pub fn new(ctx: ComponentContext, config: AroonIndicatorConfig) -> Self {
        return AroonIndicator {
            ctx: ctx.clone(),
            config,
        };
    }

    pub fn next(&mut self) -> AroonIndicatorResult {
        self.ctx.assert();

        let ctx = self.ctx.get();

        if (!ctx.at_length(self.config.length)) {
            return AroonIndicatorResult {
                up: None,
                down: None,
            };
        }

        let high = BarUtils::highest_bars(ctx.prev_highs(self.config.length), self.config.length);
        let low = BarUtils::lowest_bars(ctx.prev_lows(self.config.length), self.config.length);
        let length = self.config.length as f64;

        match (high, low) {
            (Some(high), Some(low)) => {
                let up = (high as f64 + length) / length * 100.0;
                let down = (low as f64 + length) / length * 100.0;
                return AroonIndicatorResult {
                    up: Some(up),
                    down: Some(down),
                };
            }
            _ => {
                return AroonIndicatorResult {
                    up: None,
                    down: None,
                }
            }
        }
    }
}
