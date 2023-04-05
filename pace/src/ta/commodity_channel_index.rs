use crate::{
    common::src::{AnySrc, Src, SrcKind},
    core::{
        context::Context,
        incremental::{Incremental, IncrementalDefault},
    },
    statistics::stdev::Stdev,
    strategy::trade::TradeDirection,
    ta::{
        cross::Cross,
        cross_over_threshold::CrossOverThreshold,
        cross_under_threshold::CrossUnderThreshold,
        dev::Dev,
        highest_bars::HighestBars,
        lowest_bars::LowestBars,
        moving_average::{Ma, MaKind},
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

impl Incremental<f64, f64> for Cci {
    fn next(&mut self, value: f64) -> f64 {
        let ma = self.sma.next(value);
        let dev = self.dev.next(value);

        return (value - ma) / (0.015 * dev);
    }
}
