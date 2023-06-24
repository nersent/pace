use crate::{
    common::float_series::FloatSeries,
    core::{context::Context, incremental::Incremental},
    pinescript::common::PineScriptFloat64,
};

/*pine

lwma(float src, float per)=>
    float sumw = 0
    float sum = 0
    float out = 0
    for i = 0 to per - 1
        float weight = (per - i) * per
        sumw += weight
        // sum  += nz(src[i]) * weight
        sum  += src[i] * weight
    out := sum / sumw
    out
*/
pub struct Lwma {
    pub ctx: Context,
    pub length: usize,
    src_series: FloatSeries,
}

impl Lwma {
    pub fn new(ctx: Context, length: usize) -> Self {
        Self {
            ctx: ctx.clone(),
            src_series: FloatSeries::new(ctx),
            length,
        }
    }
}

impl Incremental<f64, f64> for Lwma {
    fn next(&mut self, src: f64) -> f64 {
        self.src_series.next(src);

        let mut sumw = 0.0;
        let mut sum = 0.0;

        for i in 0..self.length {
            let weight = (self.length - i) * self.length;
            sumw += weight as f64;
            sum += self.src_series.get(i) * weight as f64;
        }

        return sum / sumw;
    }
}
