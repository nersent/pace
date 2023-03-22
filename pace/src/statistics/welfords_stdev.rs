use crate::core::{context::Context, incremental::Incremental};

use super::{common::stdev_from_var, welfords_var::WelfordsVar};
/// Calculates standard deviation using Welford's online algorithm. Has O(1) complexity.
pub struct WelfordsStdev {
    pub ctx: Context,
    variance: WelfordsVar,
}

impl WelfordsStdev {
    pub fn new(ctx: Context) -> Self {
        return Self {
            ctx: ctx.clone(),
            variance: WelfordsVar::new(ctx.clone()),
        };
    }
}

impl Incremental<f64, f64> for WelfordsStdev {
    fn next(&mut self, value: f64) -> f64 {
        return stdev_from_var(self.variance.next(value));
    }
}
