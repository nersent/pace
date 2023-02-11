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
    pub kind: SourceKind,
    ctx: ComponentContext,
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
            SourceKind::OHLC4 => ctx.ohlc4(),
            SourceKind::HLC3 => ctx.hlc3(),
            SourceKind::HL2 => ctx.hl2(),
        }
    }
}

impl TryFrom<usize> for SourceKind {
    type Error = String;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(SourceKind::Open),
            1 => Ok(SourceKind::High),
            2 => Ok(SourceKind::Low),
            3 => Ok(SourceKind::Close),
            4 => Ok(SourceKind::Volume),
            5 => Ok(SourceKind::OHLC4),
            6 => Ok(SourceKind::HLC3),
            7 => Ok(SourceKind::HL2),
            _ => Err(format!("Invalid source kind: {}", value)),
        }
    }
}
