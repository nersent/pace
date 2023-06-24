use crate::{
    common::float_series::FloatSeries,
    core::{context::Context, incremental::Incremental},
    pinescript::common::PineScriptFloat64,
};

use super::linearly_weighted_moving_average::Lwma;

/*pine

hma(float src, float per)=>
    int HalfPeriod = math.floor(per / 2)
    int HullPeriod = math.floor(math.sqrt(per))
    float out = 0
    float price1 = 2.0 * lwma(src, HalfPeriod) - lwma(src, per)
    out := lwma(price1, HullPeriod)
    out
  out
*/
pub struct Hma {
    pub ctx: Context,
    pub length: usize,
    lwma_half: Lwma,
    lwma_per: Lwma,
    lwma_hull: Lwma,
}

impl Hma {
    pub fn new(ctx: Context, length: usize) -> Self {
        assert!(length > 1, "length must be greater than 1");
        Self {
            ctx: ctx.clone(),
            lwma_half: Lwma::new(ctx.clone(), length / 2),
            lwma_per: Lwma::new(ctx.clone(), length),
            lwma_hull: Lwma::new(ctx.clone(), (length as f64).sqrt() as usize),
            length,
        }
    }
}

impl Incremental<f64, f64> for Hma {
    fn next(&mut self, src: f64) -> f64 {
        let price1 = 2.0 * self.lwma_half.next(src) - self.lwma_per.next(src);
        let lwma_hull = self.lwma_hull.next(price1);
        return lwma_hull;
    }
}
