use crate::base::{
    components::{component_context::ComponentContext, component_default::ComponentDefault},
    ta::{
        sum_component::SumComponent, swma_component::SymmetricallyWeightedMovingAverageComponent,
    },
};

pub struct RelativeVigorIndexIndicatorConfig {
    pub length: usize,
}

impl ComponentDefault for RelativeVigorIndexIndicatorConfig {
    fn default(ctx: ComponentContext) -> Self {
        Self { length: 10 }
    }
}

pub struct RelativeVigorIndexIndicatorResult {
    pub rvi: Option<f64>,
    pub sig: Option<f64>,
}

pub struct RelativeVigorIndexIndicator {
    pub config: RelativeVigorIndexIndicatorConfig,
    ctx: ComponentContext,
    swma_close_open: SymmetricallyWeightedMovingAverageComponent,
    swma_high_low: SymmetricallyWeightedMovingAverageComponent,
    sum_close_open: SumComponent,
    sum_high_low: SumComponent,
    swma_sig: SymmetricallyWeightedMovingAverageComponent,
}

impl RelativeVigorIndexIndicator {
    pub fn new(ctx: ComponentContext, config: RelativeVigorIndexIndicatorConfig) -> Self {
        return RelativeVigorIndexIndicator {
            ctx: ctx.clone(),
            swma_close_open: SymmetricallyWeightedMovingAverageComponent::new(ctx.clone()),
            swma_high_low: SymmetricallyWeightedMovingAverageComponent::new(ctx.clone()),
            sum_close_open: SumComponent::new(ctx.clone(), config.length),
            sum_high_low: SumComponent::new(ctx.clone(), config.length),
            swma_sig: SymmetricallyWeightedMovingAverageComponent::new(ctx.clone()),
            config,
        };
    }

    pub fn next(&mut self) -> RelativeVigorIndexIndicatorResult {
        self.ctx.assert();
        let ctx = self.ctx.get();

        let close = ctx.close();
        let open = ctx.open();
        let high = ctx.high();
        let low = ctx.low();

        let close_open_diff = match (close, open) {
            (Some(close), Some(open)) => Some(close - open),
            _ => None,
        };

        let high_low_diff = match (high, low) {
            (Some(high), Some(low)) => Some(high - low),
            _ => None,
        };

        let close_open_swma = self.swma_close_open.next(close_open_diff);
        let high_low_swma = self.swma_high_low.next(high_low_diff);

        let close_open_sum = self.sum_close_open.next(close_open_swma);
        let high_low_sum = self.sum_high_low.next(high_low_swma);

        let rvi = match (close_open_sum, high_low_sum) {
            (Some(close_open_sum), Some(high_low_sum)) => {
                if high_low_sum == 0.0 {
                    None
                } else {
                    Some(close_open_sum / high_low_sum)
                }
            }
            _ => None,
        };

        let sig = self.swma_sig.next(rvi);

        return RelativeVigorIndexIndicatorResult { rvi, sig };
    }
}
