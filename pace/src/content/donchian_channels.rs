use crate::{
    common::src::{AnySrc, Src, SrcKind},
    core::{
        context::Context,
        incremental::{Incremental, IncrementalDefault},
    },
    pinescript::common::ps_add,
    strategy::trade::TradeDirection,
    ta::{
        cross::Cross,
        cross_over_threshold::CrossOverThreshold,
        cross_under_threshold::CrossUnderThreshold,
        highest::Highest,
        highest_bars::HighestBars,
        lowest::Lowest,
        lowest_bars::LowestBars,
        moving_average::{AnyMa, Ma, MaKind},
    },
};

pub struct DonchianChannelsConfig {
    pub length: usize,
}

impl Default for DonchianChannelsConfig {
    fn default() -> Self {
        Self { length: 20 }
    }
}

pub struct DonchianChannelsData {
    pub upper: Option<f64>,
    pub basis: Option<f64>,
    pub lower: Option<f64>,
}

/// Ported from https://www.tradingview.com/chart/?solution=43000502253
pub struct DonchianChannels {
    pub config: DonchianChannelsConfig,
    pub ctx: Context,
    highest: Highest,
    lowest: Lowest,
}

impl DonchianChannels {
    pub fn new(ctx: Context, config: DonchianChannelsConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            highest: Highest::new(ctx.clone(), config.length),
            lowest: Lowest::new(ctx.clone(), config.length),
            config,
        };
    }
}

impl Incremental<(), DonchianChannelsData> for DonchianChannels {
    fn next(&mut self, _: ()) -> DonchianChannelsData {
        let bar = self.ctx.bar();
        let upper = self.highest.next(bar.high);
        let lower = self.lowest.next(bar.low);

        let basis = ps_add(upper, lower).map(|x| x / 2.0);

        return DonchianChannelsData {
            upper,
            basis,
            lower,
        };
    }
}
