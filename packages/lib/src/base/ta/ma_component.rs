use crate::base::{
    components::component_context::ComponentContext,
    ta::{
        ema_component::ExponentialMovingAverageComponent,
        rma_component::RunningMovingAverageComponent, sma_component::SimpleMovingAverageComponent,
    },
};

use super::ma::{MovingAverageComponentUnion, MovingAverageKind};

pub struct MovingAverageComponent {
    pub length: usize,
    pub kind: MovingAverageKind,
    ctx: ComponentContext,
    ma: MovingAverageComponentUnion,
}

impl MovingAverageComponent {
    pub fn new(ctx: ComponentContext, length: usize, kind: MovingAverageKind) -> Self {
        return MovingAverageComponent {
            length,
            ctx: ctx.clone(),
            kind,
            ma: match kind {
                MovingAverageKind::SMA => MovingAverageComponentUnion::SMA(
                    SimpleMovingAverageComponent::new(ctx.clone(), length),
                ),
                MovingAverageKind::EMA => MovingAverageComponentUnion::EMA(
                    ExponentialMovingAverageComponent::new(ctx.clone(), length),
                ),
                MovingAverageKind::RMA => MovingAverageComponentUnion::RMA(
                    RunningMovingAverageComponent::new(ctx.clone(), length),
                ),
            },
        };
    }

    pub fn next(&mut self, value: Option<f64>) -> Option<f64> {
        self.ctx.assert();
        match &mut self.ma {
            MovingAverageComponentUnion::SMA(ma) => ma.next(value),
            MovingAverageComponentUnion::EMA(ma) => ma.next(value),
            MovingAverageComponentUnion::RMA(ma) => ma.next(value),
        }
    }
}
