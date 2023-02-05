use crate::base::components::{
    common::position_component::PositionComponent, component_context::ComponentContext,
};

use super::sma_component::SimpleMovingAverageComponent;

pub struct ExponentialMovingAverageComponent {
    pub alpha: f64,
    pub length: usize,
    ctx: ComponentContext,
    sma: SimpleMovingAverageComponent,
    prev_value: Option<f64>,
}

impl ExponentialMovingAverageComponent {
    pub fn new(ctx: ComponentContext, length: usize) -> Self {
        return ExponentialMovingAverageComponent::with_alpha(
            ctx,
            length,
            2.0 / (length as f64 + 1.0),
        );
    }

    pub fn with_alpha(ctx: ComponentContext, length: usize, alpha: f64) -> Self {
        assert!(length > 0, "RecursiveEMA must have a length larger than 0");
        return ExponentialMovingAverageComponent {
            length,
            alpha,
            ctx: ctx.clone(),
            sma: SimpleMovingAverageComponent::new(ctx.clone(), length),
            prev_value: None,
        };
    }

    pub fn next(&mut self, value: Option<f64>) -> Option<f64> {
        self.ctx.on_next();
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
