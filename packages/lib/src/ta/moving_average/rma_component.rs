use crate::{
    components::component_context::ComponentContext,
    ta::moving_average::ema_component::ExponentialMovingAverageComponent,
};

pub struct RunningMovingAverageComponent {
    pub length: usize,
    ctx: ComponentContext,
    ema: ExponentialMovingAverageComponent,
}

impl RunningMovingAverageComponent {
    pub fn new(ctx: ComponentContext, length: usize) -> Self {
        assert!(length > 1, "RecursiveRMA must have a length larger than 1");
        return RunningMovingAverageComponent {
            length,
            ctx: ctx.clone(),
            ema: ExponentialMovingAverageComponent::with_alpha(
                ctx.clone(),
                length,
                1.0 / length as f64,
            ),
        };
    }

    pub fn next(&mut self, value: Option<f64>) -> Option<f64> {
        self.ctx.assert();
        return self.ema.next(value);
    }
}
