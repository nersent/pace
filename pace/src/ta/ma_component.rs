use crate::components::{component::Component, component_context::ComponentContext};

use super::{
    ema_component::EmaComponent, ma::MaKind, rma_component::RmaComponent,
    sma_component::SmaComponent,
};

pub type AnyMaComponent = Box<dyn Component<Option<f64>, Option<f64>>>;

pub struct MaComponent {
    pub length: usize,
    pub kind: MaKind,
    pub ctx: ComponentContext,
    ma: AnyMaComponent,
}

impl MaComponent {
    pub fn new(ctx: ComponentContext, kind: MaKind, length: usize) -> Self {
        return Self {
            length,
            ctx: ctx.clone(),
            kind,
            ma: Self::create_ma(ctx.clone(), kind, length),
        };
    }

    pub fn build(ctx: ComponentContext, kind: MaKind, length: usize) -> Box<Self> {
        return Box::new(Self::new(ctx, kind, length));
    }

    fn create_ma(ctx: ComponentContext, kind: MaKind, length: usize) -> AnyMaComponent {
        match kind {
            MaKind::SMA => Box::new(SmaComponent::new(ctx, length)),
            MaKind::EMA => Box::new(EmaComponent::new(ctx, length)),
            MaKind::RMA => Box::new(RmaComponent::new(ctx, length)),
        }
    }
}

impl Component<Option<f64>, Option<f64>> for MaComponent {
    fn next(&mut self, value: Option<f64>) -> Option<f64> {
        return self.ma.next(value);
    }
}
