use super::recursive_rma::RecursiveRMA;
use crate::base::component_context::ComponentContext;

pub struct RecursiveRSI {
    pub length: usize,
    ctx: ComponentContext,
    up_rma: RecursiveRMA,
    down_rma: RecursiveRMA,
    prev_input_value: Option<f64>,
}

pub struct RecursiveRSIResult {
    pub rsi: Option<f64>,
    pub up: Option<f64>,
    pub down: Option<f64>,
}

impl RecursiveRSI {
    pub fn new(ctx: ComponentContext, length: usize) -> Self {
        assert!(length > 1, "RecursiveRSI must have a length of at least 2");
        return RecursiveRSI {
            length,
            ctx: ctx.clone(),
            prev_input_value: None,
            up_rma: RecursiveRMA::new(ctx.clone(), length),
            down_rma: RecursiveRMA::new(ctx.clone(), length),
        };
    }

    pub fn next(&mut self, value: Option<f64>) -> RecursiveRSIResult {
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

        match (up, down) {
            (Some(up), Some(down)) => {
                let rs = up / down;
                let rsi = 100.0 - 100.0 / (1.0 + rs);
                return RecursiveRSIResult {
                    rsi: Some(rsi),
                    up: Some(up),
                    down: Some(down),
                };
            }
            _ => {
                return RecursiveRSIResult {
                    rsi: None,
                    down: None,
                    up: None,
                };
            }
        };
    }
}
