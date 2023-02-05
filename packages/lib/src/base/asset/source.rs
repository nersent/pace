use crate::base::components::component_context::ComponentContext;

#[derive(Debug, PartialEq, Clone)]
pub enum SourceKind {
    Open,
    High,
    Low,
    Close,
    Volume,
    OHLC4,
    HLC3,
    HL2,
}

pub struct Source {
    ctx: ComponentContext,
    kind: SourceKind,
}

impl Source {
    pub fn from_kind(ctx: ComponentContext, kind: SourceKind) -> Self {
        return Source { ctx, kind };
    }

    pub fn get(&self) -> Option<f64> {
        let ctx = self.ctx.get();
        match self.kind {
            SourceKind::Open => ctx.open(),
            SourceKind::High => ctx.high(),
            SourceKind::Low => ctx.low(),
            SourceKind::Close => ctx.close(),
            SourceKind::Volume => ctx.volume(),
            SourceKind::OHLC4 => todo!(),
            SourceKind::HLC3 => ctx.hlc3(),
            SourceKind::HL2 => ctx.hl2(),
        }
    }
}
