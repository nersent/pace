use crate::{
    common::src::{AnySrc, Src, SrcKind},
    core::{
        context::Context,
        incremental::{Incremental, IncrementalDefault},
    },
    strategy::trade::TradeDirection,
    ta::{
        average_true_range::Atr,
        cross::Cross,
        cross_over_threshold::CrossOverThreshold,
        cross_under_threshold::CrossUnderThreshold,
        highest::Highest,
        highest_bars::HighestBars,
        lowest::Lowest,
        lowest_bars::LowestBars,
        moving_average::{AnyMa, Ma, MaKind},
        sum::Sum,
    },
};

pub struct ChoppinessIndexConfig {
    pub length: usize,
}

impl Default for ChoppinessIndexConfig {
    fn default() -> Self {
        Self { length: 14 }
    }
}

/// Choppiness Index Indicator.
///
/// Ported from https://www.tradingview.com/chart/?solution=43000501980
pub struct ChoppinessIndex {
    pub config: ChoppinessIndexConfig,
    pub ctx: Context,
    atr: Atr,
    atr_sum: Sum,
    log10_length: f64,
    highest: Highest,
    lowest: Lowest,
}

impl ChoppinessIndex {
    pub fn new(ctx: Context, config: ChoppinessIndexConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            atr: Atr::new(ctx.clone(), 1),
            atr_sum: Sum::new(ctx.clone(), config.length),
            log10_length: f64::log10(config.length as f64),
            highest: Highest::new(ctx.clone(), config.length),
            lowest: Lowest::new(ctx.clone(), config.length),
            config,
        };
    }
}

impl Incremental<(), Option<f64>> for ChoppinessIndex {
    fn next(&mut self, _: ()) -> Option<f64> {
        let atr = self.atr.next(());
        let atr_sum = self.atr_sum.next(atr);

        let highest = self.highest.next(self.ctx.bar.high());
        let lowest = self.lowest.next(self.ctx.bar.low());

        let chop: Option<f64> = match (atr_sum, highest, lowest) {
            (Some(atr_sum), Some(highest), Some(lowest)) => {
                let diff = highest - lowest;
                if diff == 0.0 {
                    None
                } else {
                    Some(100.0 * f64::log10(atr_sum / diff) / self.log10_length)
                }
            }
            _ => None,
        };

        return chop;
    }
}
