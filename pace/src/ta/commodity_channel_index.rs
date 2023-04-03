use crate::{
    common::src::{AnySrc, Src, SrcKind},
    core::{
        context::Context,
        incremental::{Incremental, IncrementalDefault},
    },
    pinescript::common::{ps_diff, ps_div},
    statistics::stdev::Stdev,
    strategy::trade::TradeDirection,
    ta::{
        cross::Cross,
        cross_over_threshold::CrossOverThreshold,
        cross_under_threshold::CrossUnderThreshold,
        dev::Dev,
        highest_bars::HighestBars,
        lowest_bars::LowestBars,
        moving_average::{AnyMa, Ma, MaKind},
        simple_moving_average::Sma,
    },
};

pub struct Cci {
    pub length: usize,
    pub ctx: Context,
    sma: Sma,
    dev: Dev,
}

impl Cci {
    pub fn new(ctx: Context, length: usize) -> Self {
        return Self {
            ctx: ctx.clone(),
            length,
            sma: Sma::new(ctx.clone(), length),
            dev: Dev::new(ctx.clone(), length),
        };
    }
}

impl Incremental<Option<f64>, Option<f64>> for Cci {
    fn next(&mut self, src: Option<f64>) -> Option<f64> {
        let ma = self.sma.next(src);
        let dev = self.dev.next(src);

        if src.is_none() || ma.is_none() || dev.is_none() {
            return None;
        }

        let src = src.unwrap();
        let ma = ma.unwrap();
        let dev = dev.unwrap();

        let cci = (src - ma) / (0.015 * dev);

        return Some(cci);
    }
}
