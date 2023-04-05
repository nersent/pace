use crate::{
    common::src::{AnySrc, Src, SrcKind},
    core::{
        context::Context,
        incremental::{Incremental, IncrementalDefault},
    },
    strategy::trade::TradeDirection,
    ta::{
        average_true_range::Atr, cross::Cross, cross_over_threshold::CrossOverThreshold,
        cross_under_threshold::CrossUnderThreshold, highest::Highest, highest_bars::HighestBars,
        lowest::Lowest, lowest_bars::LowestBars,
    },
};

pub struct ChandeKrollStopConfig {
    pub p: usize,
    pub q: usize,
    pub x: f64,
}

impl Default for ChandeKrollStopConfig {
    fn default() -> Self {
        Self {
            p: 10,
            x: 1.0,
            q: 9,
        }
    }
}

pub struct ChandeKrollStopData {
    pub first_high_stop: f64,
    pub first_low_stop: f64,
    pub stop_long: f64,
    pub stop_short: f64,
}

/// Ported from https://www.tradingview.com/chart/?solution=43000589105
pub struct ChandeKrollStop {
    pub config: ChandeKrollStopConfig,
    pub ctx: Context,
    atr: Atr,
    first_high_stop_highest: Highest,
    first_low_stop_lowest: Lowest,
    stop_short_highest: Highest,
    stop_long_lowest: Lowest,
}

impl ChandeKrollStop {
    pub fn new(ctx: Context, config: ChandeKrollStopConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            atr: Atr::new(ctx.clone(), config.p),
            first_high_stop_highest: Highest::new(ctx.clone(), config.p),
            first_low_stop_lowest: Lowest::new(ctx.clone(), config.p),
            stop_short_highest: Highest::new(ctx.clone(), config.q),
            stop_long_lowest: Lowest::new(ctx.clone(), config.q),
            config,
        };
    }
}

impl Incremental<(), ChandeKrollStopData> for ChandeKrollStop {
    fn next(&mut self, _: ()) -> ChandeKrollStopData {
        let atr = self.atr.next(());
        let first_high_stop_highest = self.first_high_stop_highest.next(self.ctx.bar.high());
        let first_low_stop_lowest = self.first_low_stop_lowest.next(self.ctx.bar.low());

        let first_high_stop = first_high_stop_highest - self.config.x * atr;
        let first_low_stop = first_low_stop_lowest + self.config.x * atr;

        let stop_short = self.stop_short_highest.next(first_high_stop);
        let stop_long = self.stop_long_lowest.next(first_low_stop);

        return ChandeKrollStopData {
            first_high_stop,
            first_low_stop,
            stop_short,
            stop_long,
        };
    }
}
