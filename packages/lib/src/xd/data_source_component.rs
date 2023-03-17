use super::{component::Component, source_kind::SourceKind};
use crate::base::components::component_context::ComponentContext;

pub type AnySourceComponent = Box<dyn Component<(), Option<f64>>>;

pub struct SourceComponent {
    pub kind: SourceKind,
    pub ctx: ComponentContext,
    delegate: Box<dyn FnMut() -> Option<f64>>,
}

impl SourceComponent {
    pub fn new(ctx: ComponentContext, kind: SourceKind) -> Self {
        return Self {
            ctx: ctx.clone(),
            kind,
            delegate: Self::create_delegate(ctx.clone(), kind),
        };
    }

    fn create_delegate(ctx: ComponentContext, kind: SourceKind) -> Box<dyn FnMut() -> Option<f64>> {
        match kind {
            SourceKind::Open => Box::new(move || ctx.get().open()),
            SourceKind::High => Box::new(move || ctx.get().high()),
            SourceKind::Low => Box::new(move || ctx.get().low()),
            SourceKind::Close => Box::new(move || ctx.get().close()),
            SourceKind::Volume => Box::new(move || ctx.get().volume()),
            SourceKind::OHLC4 => Box::new(move || ctx.get().ohlc4()),
            SourceKind::HLC3 => Box::new(move || ctx.get().hlc3()),
            SourceKind::HL2 => Box::new(move || ctx.get().hl2()),
        }
    }
}

impl Component<(), Option<f64>> for SourceComponent {
    fn next(&mut self, _: ()) -> Option<f64> {
        return self.delegate.as_mut()();
    }
}
