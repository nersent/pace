use crate::base::{
    components::{component_context::ComponentContext, component_default::ComponentDefault},
    ta::{
        highest_bars_component::HighestBarsComponent, lowest_bars_component::LowestBarsComponent,
    },
};

pub struct AroonIndicatorConfig {
    pub length: usize,
}

impl ComponentDefault for AroonIndicatorConfig {
    fn default(ctx: ComponentContext) -> Self {
        Self { length: 14 }
    }
}

pub struct AroonIndicator {
    config: AroonIndicatorConfig,
    ctx: ComponentContext,
    highest_bars: HighestBarsComponent,
    lowest_bars: LowestBarsComponent,
}

pub struct AroonIndicatorResult {
    pub up: Option<f64>,
    pub down: Option<f64>,
}

impl AroonIndicator {
    pub fn new(ctx: ComponentContext, config: AroonIndicatorConfig) -> Self {
        return AroonIndicator {
            ctx: ctx.clone(),
            highest_bars: HighestBarsComponent::new(ctx.clone(), config.length),
            lowest_bars: LowestBarsComponent::new(ctx.clone(), config.length),
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

        let high = self.highest_bars.next();
        let low = self.lowest_bars.next();

        let length = self.config.length as f64;

        let up = high.map(|high| (high as f64 + length) / length * 100.0);
        let down = low.map(|low| (low as f64 + length) / length * 100.0);

        return AroonIndicatorResult { up, down };
    }
}
