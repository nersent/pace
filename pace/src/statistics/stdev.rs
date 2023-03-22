use crate::core::{context::Context, incremental::Incremental};

use super::{common::stdev_from_var, var::Var};

/// Standard deviation.
///
/// For O(1) complexity, use `fast`. By default it's `false`.
///
/// Compared to `ta::stdev`, this component calculates stdev based on entire history of values.
///
/// Not the same as PineScript `ta.stdev`.
pub struct Stdev {
    pub ctx: Context,
    pub fast: bool,
    variance: Var,
}

impl Stdev {
    pub fn build(ctx: Context, fast: bool) -> Self {
        return Self {
            ctx: ctx.clone(),
            fast,
            variance: Var::build(ctx.clone(), fast),
        };
    }

    pub fn new(ctx: Context) -> Self {
        return Self::build(ctx, false);
    }

    pub fn fast(ctx: Context) -> Self {
        return Self::build(ctx, true);
    }
}

impl Incremental<f64, f64> for Stdev {
    fn next(&mut self, value: f64) -> f64 {
        return stdev_from_var(self.variance.next(value));
    }
}
