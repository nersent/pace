use crate::{
    components::{component::Component, component_context::ComponentContext},
    ta::{
        atr_component::AtrComponent, highest_component::HighestComponent,
        lowest_component::LowestComponent,
    },
};

pub struct CksIndicatorConfig {
    pub p: usize,
    pub q: usize,
    pub x: f64,
}

impl Default for CksIndicatorConfig {
    fn default() -> Self {
        Self {
            p: 10,
            x: 1.0,
            q: 9,
        }
    }
}

pub struct CksIndicatorData {
    pub first_high_stop: Option<f64>,
    pub first_low_stop: Option<f64>,
    pub stop_long: Option<f64>,
    pub stop_short: Option<f64>,
}

/// Chande Kroll Stop Indicator.
///
/// Ported from https://www.tradingview.com/chart/?solution=43000589105
pub struct CksIndicator {
    pub config: CksIndicatorConfig,
    pub ctx: ComponentContext,
    atr: AtrComponent,
    first_high_stop_highest: HighestComponent,
    first_low_stop_lowest: LowestComponent,
    stop_short_highest: HighestComponent,
    stop_long_lowest: LowestComponent,
}

impl CksIndicator {
    pub fn new(ctx: ComponentContext, config: CksIndicatorConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            atr: AtrComponent::new(ctx.clone(), config.p),
            first_high_stop_highest: HighestComponent::new(ctx.clone(), config.p),
            first_low_stop_lowest: LowestComponent::new(ctx.clone(), config.p),
            stop_short_highest: HighestComponent::new(ctx.clone(), config.q),
            stop_long_lowest: LowestComponent::new(ctx.clone(), config.q),
            config,
        };
    }
}

impl Component<(), CksIndicatorData> for CksIndicator {
    fn next(&mut self, _: ()) -> CksIndicatorData {
        let atr = self.atr.next(());

        let first_high_stop_highest = self.first_high_stop_highest.next(self.ctx.high());
        let first_low_stop_lowest = self.first_low_stop_lowest.next(self.ctx.low());

        let (first_high_stop, first_low_stop) =
            match (first_high_stop_highest, first_low_stop_lowest, atr) {
                (Some(first_high_stop_highest), Some(first_low_stop_lowest), Some(atr)) => {
                    let first_high_stop = first_high_stop_highest - self.config.x * atr;
                    let first_low_stop = first_low_stop_lowest + self.config.x * atr;
                    (Some(first_high_stop), Some(first_low_stop))
                }
                _ => (None, None),
            };

        let stop_short = self.stop_short_highest.next(first_high_stop);
        let stop_long = self.stop_long_lowest.next(first_low_stop);

        return CksIndicatorData {
            first_high_stop,
            first_low_stop,
            stop_short,
            stop_long,
        };
    }
}
