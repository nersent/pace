use crate::core::{context::Context, incremental::Incremental};

use super::{
    exponential_moving_average::Ema, running_moving_average::Rma, simple_moving_average::Sma,
    symmetrically_weighted_moving_average::Swma,
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MaKind {
    SMA,
    EMA,
    RMA,
    SWMA,
}

/// Any incremental moving average.
pub type AnyMa = Box<dyn Incremental<Option<f64>, Option<f64>>>;

/// A simplified way of creating a moving average component.
pub struct Ma {
    pub length: usize,
    pub kind: MaKind,
    pub ctx: Context,
    ma: AnyMa,
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

    fn create_ma(ctx: Context, kind: MaKind, length: usize) -> AnyMa {
        match kind {
            MaKind::SMA => Box::new(Sma::new(ctx, length)),
            MaKind::EMA => Box::new(Ema::new(ctx, length)),
            MaKind::RMA => Box::new(Rma::new(ctx, length)),
            MaKind::SWMA => Box::new(Swma::new(ctx)),
        }
    }
}

impl Incremental<Option<f64>, Option<f64>> for Ma {
    fn next(&mut self, value: Option<f64>) -> Option<f64> {
        return self.ma.next(value);
    }
}
