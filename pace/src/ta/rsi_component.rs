use crate::components::{component::Component, component_context::ComponentContext};

use super::rma_component::RmaComponent;

pub static RSI_MIN_VALUE: f64 = 0.0;
pub static RSI_MAX_VALUE: f64 = 100.0;

pub struct RsiData {
    pub up: Option<f64>,
    pub down: Option<f64>,
}

/// Relative Strength Index. It is calculated using the `ta.rma()` of upward and downward changes of `source` over the last `length` bars.
///
/// Same as PineScript `ta.rsi(src)`. Similar to `ta.rsi(src, length)`, but `length` is fixed and set on initialization.
pub struct RsiComponent {
    pub length: usize,
    pub ctx: ComponentContext,
    up_rma: RmaComponent,
    down_rma: RmaComponent,
    prev_input_value: Option<f64>,
    pub data: RsiData,
}

impl RsiComponent {
    pub fn new(ctx: ComponentContext, length: usize) -> Self {
        assert!(length >= 2, "RsiComponent must have a length of at least 2");
        return Self {
            length,
            ctx: ctx.clone(),
            prev_input_value: None,
            up_rma: RmaComponent::new(ctx.clone(), length),
            down_rma: RmaComponent::new(ctx.clone(), length),
            data: RsiData {
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

        self.data.up = up;
        self.data.down = down;

        return Some(rsi);
    }
}
