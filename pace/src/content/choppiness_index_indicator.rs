use crate::{
    components::{component::Component, component_context::ComponentContext},
    ta::{
        atr_component::AtrComponent, highest_component::HighestComponent,
        lowest_component::LowestComponent, sum_component::SumComponent,
    },
};

pub struct CiIndicatorConfig {
    pub length: usize,
}

impl Default for CiIndicatorConfig {
    fn default() -> Self {
        Self { length: 14 }
    }
}

pub struct CiIndicator {
    pub config: CiIndicatorConfig,
    pub ctx: ComponentContext,
    atr: AtrComponent,
    atr_sum: SumComponent,
    log10_length: f64,
    highest: HighestComponent,
    lowest: LowestComponent,
}

impl CiIndicator {
    pub fn new(ctx: ComponentContext, config: CiIndicatorConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            atr: AtrComponent::new(ctx.clone(), 1),
            atr_sum: SumComponent::new(ctx.clone(), config.length),
            log10_length: f64::log10(config.length as f64),
            highest: HighestComponent::new(ctx.clone(), config.length),
            lowest: LowestComponent::new(ctx.clone(), config.length),
            config,
        };
    }
}

impl Component<(), Option<f64>> for CiIndicator {
    fn next(&mut self, _: ()) -> Option<f64> {
        let atr = self.atr.next(());
        let atr_sum = self.atr_sum.next(atr);

        let highest = self.highest.next(self.ctx.high());
        let lowest = self.lowest.next(self.ctx.low());

        let chop: Option<f64> = match (atr_sum, highest, lowest) {
            (Some(atr_sum), Some(highest), Some(lowest)) => {
                let diff = highest - lowest;
                if diff == 0.0 {
                    None
                } else {
                    Some(100.0 * f64::log10(atr_sum / diff) / self.log10_length)
                }
            }
            _ => None,
        };

        return chop;
    }
}
