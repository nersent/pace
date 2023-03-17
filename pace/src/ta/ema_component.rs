use crate::components::{component::Component, component_context::ComponentContext};

use super::sma_component::SmaComponent;

/// Exponential moving average.
pub struct EmaComponent {
    pub alpha: f64,
    pub length: usize,
    pub ctx: ComponentContext,
    sma: SmaComponent,
    prev_value: Option<f64>,
}

impl EmaComponent {
    pub fn new(ctx: ComponentContext, length: usize) -> Self {
        return Self::with_alpha(ctx, length, 2.0 / (length as f64 + 1.0));
    }

    pub fn with_alpha(ctx: ComponentContext, length: usize, alpha: f64) -> Self {
        assert!(length > 0, "EmaComponent must have a length larger than 0");
        return Self {
            length,
            alpha,
            ctx: ctx.clone(),
            sma: SmaComponent::new(ctx.clone(), length),
            prev_value: None,
        };
    }
}

impl Component<Option<f64>, Option<f64>> for EmaComponent {
    fn next(&mut self, value: Option<f64>) -> Option<f64> {
        if self.length == 1 {
            return value;
        }
        if !self.ctx.at_length(self.length - 1) {
            self.sma.next(value);
            return None;
        }
        match self.prev_value {
            Some(prev_value) => {
                let ema = self.alpha * value.unwrap() + (1.0 - self.alpha) * prev_value;
                self.prev_value = Some(ema);
                return self.prev_value;
            }
            None => {
                self.prev_value = self.sma.next(value);
                return self.prev_value;
            }
        }
    }
}
