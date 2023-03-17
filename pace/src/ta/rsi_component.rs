use crate::components::{component::Component, component_context::ComponentContext};

use super::rma_component::RmaComponent;

pub struct RsiMetadata {
    pub up: Option<f64>,
    pub down: Option<f64>,
}

pub struct RsiComponent {
    pub length: usize,
    pub ctx: ComponentContext,
    up_rma: RmaComponent,
    down_rma: RmaComponent,
    prev_input_value: Option<f64>,
    pub metadata: RsiMetadata,
}

pub static RSI_MIN_VALUE: f64 = 0.0;
pub static RSI_MAX_VALUE: f64 = 100.0;

impl RsiComponent {
    pub fn new(ctx: ComponentContext, length: usize) -> Self {
        assert!(length > 1, "RsiComponent must have a length of at least 2");
        return Self {
            length,
            ctx: ctx.clone(),
            prev_input_value: None,
            up_rma: RmaComponent::new(ctx.clone(), length),
            down_rma: RmaComponent::new(ctx.clone(), length),
            metadata: RsiMetadata {
                up: None,
                down: None,
            },
        };
    }
}

impl Component<Option<f64>, Option<f64>> for RsiComponent {
    fn next(&mut self, value: Option<f64>) -> Option<f64> {
        let (up_change, down_change): (Option<f64>, Option<f64>) =
            match (self.prev_input_value, value) {
                (Some(prev_input_value), Some(value)) => {
                    let change = value - prev_input_value;
                    (Some(f64::max(change, 0.0)), Some(-f64::min(change, 0.0)))
                }
                _ => (None, None),
            };

        let up = self.up_rma.next(up_change);
        let down = self.down_rma.next(down_change);

        self.prev_input_value = value;

        if up.is_none() || down.is_none() {
            return None;
        }

        let rs = up.unwrap() / down.unwrap();
        let rsi = 100.0 - 100.0 / (1.0 + rs);

        self.metadata.up = up;
        self.metadata.down = down;

        return Some(rsi);
    }
}
