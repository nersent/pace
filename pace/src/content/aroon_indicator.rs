use crate::{
    components::{component::Component, component_context::ComponentContext},
    ta::{
        highest_bars_component::HighestBarsComponent, lowest_bars_component::LowestBarsComponent,
    },
};

pub static AROON_MIN_VALUE: f64 = 0.0;
pub static AROON_MAX_VALUE: f64 = 100.0;

pub struct AroonIndicatorConfig {
    pub length: usize,
}

impl Default for AroonIndicatorConfig {
    fn default() -> Self {
        Self { length: 14 }
    }
}

pub struct AroonIndicator {
    pub config: AroonIndicatorConfig,
    pub ctx: ComponentContext,
    highest_bars: HighestBarsComponent,
    lowest_bars: LowestBarsComponent,
}

pub struct AroonIndicatorData {
    pub up: Option<f64>,
    pub down: Option<f64>,
}

impl AroonIndicator {
    pub fn new(ctx: ComponentContext, config: AroonIndicatorConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            highest_bars: HighestBarsComponent::new(ctx.clone(), config.length),
            lowest_bars: LowestBarsComponent::new(ctx.clone(), config.length),
            config,
        };
    }
}

impl Component<(), AroonIndicatorData> for AroonIndicator {
    fn next(&mut self, _: ()) -> AroonIndicatorData {
        if !self.ctx.at_length(self.config.length) {
            return AroonIndicatorData {
                up: None,
                down: None,
            };
        }

        let high = self.highest_bars.next(());
        let low = self.lowest_bars.next(());

        let length = self.config.length as f64;

        let up = high.map(|high| (high as f64 + length) / length * 100.0);
        let down = low.map(|low| (low as f64 + length) / length * 100.0);

        return AroonIndicatorData { up, down };
    }
}
