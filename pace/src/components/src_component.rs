use crate::{
    components::{component::Component, component_context::ComponentContext},
    ta::ma::{hl2, hlc3, ohlc4},
};

use super::src_kind::SrcKind;

pub type AnySrcComponent = Box<dyn Component<(), Option<f64>>>;

pub struct SrcComponent {
    pub kind: SrcKind,
    pub ctx: ComponentContext,
    delegate: Box<dyn FnMut() -> Option<f64>>,
}

impl SrcComponent {
    pub fn new(ctx: ComponentContext, kind: SrcKind) -> Self {
        return Self {
            ctx: ctx.clone(),
            kind,
            delegate: Self::create_delegate(ctx.clone(), kind),
        };
    }

    pub fn build(ctx: ComponentContext, kind: SrcKind) -> Box<Self> {
        return Box::new(Self::new(ctx, kind));
    }

    fn create_delegate(ctx: ComponentContext, kind: SrcKind) -> Box<dyn FnMut() -> Option<f64>> {
        match kind {
            SrcKind::Open => Box::new(move || ctx.open()),
            SrcKind::High => Box::new(move || ctx.high()),
            SrcKind::Low => Box::new(move || ctx.low()),
            SrcKind::Close => Box::new(move || ctx.close()),
            SrcKind::Volume => Box::new(move || ctx.volume()),
            SrcKind::OHLC4 => Box::new(move || {
                Some(ohlc4(
                    ctx.open().unwrap(),
                    ctx.high().unwrap(),
                    ctx.low().unwrap(),
                    ctx.close().unwrap(),
                ))
            }),
            SrcKind::HLC3 => Box::new(move || {
                Some(hlc3(
                    ctx.high().unwrap(),
                    ctx.low().unwrap(),
                    ctx.close().unwrap(),
                ))
            }),
            SrcKind::HL2 => Box::new(move || Some(hl2(ctx.high().unwrap(), ctx.low().unwrap()))),
        }
    }
}

impl Component<(), Option<f64>> for SrcComponent {
    fn next(&mut self, _: ()) -> Option<f64> {
        return self.delegate.as_mut()();
    }
}
