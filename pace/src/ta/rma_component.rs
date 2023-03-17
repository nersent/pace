use crate::components::{component::Component, component_context::ComponentContext};

use super::ema_component::EmaComponent;

/// Running Moving Average. Used in RSI. It is the exponentially weighted moving average with alpha = 1 / length.
///
/// Same as PineScript `ta.rma(src)`. Similar to `ta.rma(src, length)`, but `length` is fixed and set on initialization.
pub struct RmaComponent {
    pub length: usize,
    pub ctx: ComponentContext,
    ema: EmaComponent,
}

impl RmaComponent {
    pub fn new(ctx: ComponentContext, length: usize) -> Self {
        assert!(length >= 1, "RmaComponent must have a length of at least 1");
        return Self {
            length,
            ctx: ctx.clone(),
            ema: EmaComponent::with_alpha(ctx.clone(), length, 1.0 / length as f64),
        };
    }
}

impl Component<Option<f64>, Option<f64>> for RmaComponent {
    fn next(&mut self, value: Option<f64>) -> Option<f64> {
        return self.ema.next(value);
    }
}
