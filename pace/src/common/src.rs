use crate::core::{context::Context, incremental::Incremental};

/// Any data source provider;
pub type AnySrc = Box<dyn Incremental<(), Option<f64>>>;

#[derive(Clone, Copy)]
pub enum SrcKind {
    Open,
    High,
    Low,
    Close,
    Volume,
    OHLC4,
    HLC3,
    HL2,
}

pub fn ohlc4(open: f64, high: f64, low: f64, close: f64) -> f64 {
    return (open + high + low + close) / 4.0;
}

pub fn hlc3(high: f64, low: f64, close: f64) -> f64 {
    return (high + low + close) / 3.0;
}

pub fn hl2(high: f64, low: f64) -> f64 {
    return (high + low) / 2.0;
}

pub struct Src {
    pub kind: SrcKind,
    pub ctx: Context,
    delegate: Box<dyn FnMut() -> Option<f64>>,
}

impl Src {
    pub fn new(ctx: Context, kind: SrcKind) -> Self {
        return Self {
            ctx: ctx.clone(),
            kind,
            delegate: Self::create_delegate(ctx.clone(), kind),
        };
    }

    fn create_delegate(ctx: Context, kind: SrcKind) -> Box<dyn FnMut() -> Option<f64>> {
        match kind {
            SrcKind::Open => Box::new(move || ctx.bar.open()),
            SrcKind::High => Box::new(move || ctx.bar.high()),
            SrcKind::Low => Box::new(move || ctx.bar.low()),
            SrcKind::Close => Box::new(move || ctx.bar.close()),
            SrcKind::Volume => Box::new(move || ctx.bar.volume()),
            SrcKind::OHLC4 => Box::new(move || {
                Some(ohlc4(
                    ctx.bar.open().unwrap(),
                    ctx.bar.high().unwrap(),
                    ctx.bar.low().unwrap(),
                    ctx.bar.close().unwrap(),
                ))
            }),
            SrcKind::HLC3 => Box::new(move || {
                Some(hlc3(
                    ctx.bar.high().unwrap(),
                    ctx.bar.low().unwrap(),
                    ctx.bar.close().unwrap(),
                ))
            }),
            SrcKind::HL2 => {
                Box::new(move || Some(hl2(ctx.bar.high().unwrap(), ctx.bar.low().unwrap())))
            }
        }
    }
}

impl Incremental<(), Option<f64>> for Src {
    fn next(&mut self, _: ()) -> Option<f64> {
        return self.delegate.as_mut()();
    }
}
