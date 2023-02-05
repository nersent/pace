use crate::base::components::component_context::ComponentContext;

use super::{rma_component::RunningMovingAverageComponent, tr_component::TrueRangeComponent};

pub struct AverageTrueRangeComponent {
    pub length: usize,
    ctx: ComponentContext,
    tr: TrueRangeComponent,
    rma: RunningMovingAverageComponent,
}

impl AverageTrueRangeComponent {
    pub fn new(ctx: ComponentContext, length: usize) -> Self {
        assert!(
            length > 0,
            "AverageTrueRangeComponent must have a length of at least 1"
        );
        return AverageTrueRangeComponent {
            ctx: ctx.clone(),
            length,
            tr: TrueRangeComponent::new(ctx.clone(), true),
            rma: RunningMovingAverageComponent::new(ctx.clone(), length),
        };
    }

    pub fn next(&mut self) -> Option<f64> {
        let true_range = self.tr.next();
        let atr = self.rma.next(true_range);
        return atr;
    }
}
