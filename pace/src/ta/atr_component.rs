use crate::components::{component::Component, component_context::ComponentContext};

use super::{rma_component::RmaComponent, tr_component::TrComponent};

/// Average true range.
pub struct AtrComponent {
    pub length: usize,
    pub ctx: ComponentContext,
    tr: TrComponent,
    rma: RmaComponent,
}

impl AtrComponent {
    pub fn new(ctx: ComponentContext, length: usize) -> Self {
        assert!(length > 0, "AtrComponent must have a length of at least 1");
        return Self {
            ctx: ctx.clone(),
            length,
            tr: TrComponent::new(ctx.clone(), true),
            rma: RmaComponent::new(ctx.clone(), length),
        };
    }
}

impl Component<(), Option<f64>> for AtrComponent {
    fn next(&mut self, _: ()) -> Option<f64> {
        let true_range = self.tr.next(());
        let atr = self.rma.next(true_range);
        return atr;
    }
}
