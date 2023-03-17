use crate::components::{component::Component, component_context::ComponentContext};

use super::{rma_component::RmaComponent, tr_component::TrComponent};

/// Average True Range.
///
/// Same as PineScript `ta.atr(src)`. Similar to `ta.atr(src, length)`, but `length` is fixed and set on initialization.
pub struct AtrComponent {
    pub length: usize,
    pub ctx: ComponentContext,
    tr: TrComponent,
    rma: RmaComponent,
}

impl AtrComponent {
    pub fn new(ctx: ComponentContext, length: usize) -> Self {
        assert!(length >= 1, "AtrComponent must have a length of at least 1");
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
