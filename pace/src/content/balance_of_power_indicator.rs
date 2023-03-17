use crate::components::{component::Component, component_context::ComponentContext};

pub static BOP_MIN_VALUE: f64 = -1.0;
pub static BOP_MAX_VALUE: f64 = 1.0;

pub struct BopIndicator {
    pub ctx: ComponentContext,
}

impl BopIndicator {
    pub fn new(ctx: ComponentContext) -> Self {
        return Self { ctx };
    }
}

impl Component<(), Option<f64>> for BopIndicator {
    fn next(&mut self, _: ()) -> Option<f64> {
        let close = self.ctx.close();
        let open = self.ctx.open();
        let high = self.ctx.high();
        let low = self.ctx.low();

        let value = match (close, open, high, low) {
            (Some(close), Some(open), Some(high), Some(low)) => {
                if high == low {
                    return None;
                }

                return Some((close - open) / (high - low));
            }
            _ => None,
        };

        return value;
    }
}
