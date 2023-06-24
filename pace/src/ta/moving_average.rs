use crate::{
    common::src::AnyProcessor,
    core::{context::Context, incremental::Incremental},
};

use super::{
    exponential_moving_average::Ema, hull_moving_average::Hma, running_moving_average::Rma,
    simple_moving_average::Sma, symmetrically_weighted_moving_average::Swma,
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MaKind {
    SMA,
    EMA,
    RMA,
    SWMA,
    HMA,
}

impl Into<&'static str> for MaKind {
    fn into(self) -> &'static str {
        return match self {
            MaKind::SMA => "sma",
            MaKind::EMA => "ema",
            MaKind::RMA => "rma",
            MaKind::SWMA => "swma",
            MaKind::HMA => "hma",
        };
    }
}

/// A simplified way of creating a moving average component.
pub struct Ma {
    pub length: usize,
    pub kind: MaKind,
    pub ctx: Context,
    ma: AnyProcessor,
}

impl Ma {
    pub fn new(ctx: Context, kind: MaKind, length: usize) -> Self {
        return Self {
            length,
            ctx: ctx.clone(),
            kind,
            ma: Self::create_ma(ctx.clone(), kind, length),
        };
    }

    fn create_ma(ctx: Context, kind: MaKind, length: usize) -> AnyProcessor {
        match kind {
            MaKind::SMA => Box::new(Sma::new(ctx, length)),
            MaKind::EMA => Box::new(Ema::new(ctx, length)),
            MaKind::RMA => Box::new(Rma::new(ctx, length)),
            MaKind::SWMA => Box::new(Swma::new(ctx)),
            MaKind::HMA => Box::new(Hma::new(ctx, length)),
        }
    }
}

impl Incremental<f64, f64> for Ma {
    fn next(&mut self, value: f64) -> f64 {
        return self.ma.next(value);
    }
}
