use crate::base::{
    components::{component_context::ComponentContext, component_default::ComponentDefault},
    ta::{
        atr_component::AverageTrueRangeComponent,
        bars::{compute_highest, compute_lowest},
        sum_component::SumComponent,
    },
};

pub struct ChoppinessIndexIndicatorConfig {
    pub length: usize,
}

impl ComponentDefault for ChoppinessIndexIndicatorConfig {
    fn default(ctx: ComponentContext) -> Self {
        Self { length: 14 }
    }
}

pub struct ChoppinessIndexIndicator {
    pub config: ChoppinessIndexIndicatorConfig,
    ctx: ComponentContext,
    atr: AverageTrueRangeComponent,
    atr_sum: SumComponent,
    log10_length: f64,
}

impl ChoppinessIndexIndicator {
    pub fn new(ctx: ComponentContext, config: ChoppinessIndexIndicatorConfig) -> Self {
        return ChoppinessIndexIndicator {
            ctx: ctx.clone(),
            atr: AverageTrueRangeComponent::new(ctx.clone(), 1),
            atr_sum: SumComponent::new(ctx.clone(), config.length),
            log10_length: f64::log10(config.length as f64),
            config,
        };
    }

    pub fn next(&mut self) -> Option<f64> {
        self.ctx.assert();
        let ctx = self.ctx.get();

        let atr = self.atr.next();
        let atr_sum = self.atr_sum.next(atr);

        if (atr_sum.is_none() || !ctx.at_length(self.config.length)) {
            return None;
        }

        let highest = compute_highest(ctx.prev_highs(self.config.length));
        let lowest = compute_lowest(ctx.prev_lows(self.config.length));

        let chop: Option<f64> = match (highest, lowest) {
            (Some(highest), Some(lowest)) => {
                let diff = highest - lowest;
                if diff == 0.0 {
                    None
                } else {
                    Some(100.0 * f64::log10(atr_sum.unwrap() / diff) / self.log10_length)
                }
            }
            _ => None,
        };

        return chop;
    }
}
