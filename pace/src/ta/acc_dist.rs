use crate::{
    core::{context::Context, incremental::Incremental},
    pinescript::common::PineScriptFloat64,
};

pub struct AccDist {
    pub ctx: Context,
    prev_ad: f64,
}

impl AccDist {
    pub fn new(ctx: Context) -> Self {
        return Self {
            ctx,
            prev_ad: f64::NAN,
        };
    }
}

impl Incremental<(), f64> for AccDist {
    fn next(&mut self, _: ()) -> f64 {
        let bar = &self.ctx.bar;
        let close = bar.close();
        let high = bar.high();
        let low = bar.low();
        let volume = bar.volume();

        let mfm = ((close - low) - (high - close)) / (high - low);
        let mfv = mfm * volume;

        let ad = self.prev_ad.ps_nz() + mfv.ps_nz();

        self.prev_ad = ad;

        return ad;
    }
}
