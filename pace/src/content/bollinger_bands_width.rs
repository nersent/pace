use crate::{
    common::src::{AnySrc, Src, SrcKind},
    core::{
        context::Context,
        incremental::{Incremental, IncrementalDefault},
    },
    strategy::trade::TradeDirection,
    ta::{
        cross::Cross,
        cross_over_threshold::CrossOverThreshold,
        cross_under_threshold::CrossUnderThreshold,
        highest_bars::HighestBars,
        lowest_bars::LowestBars,
        moving_average::{AnyMa, Ma, MaKind},
        simple_moving_average::Sma,
        stdev::Stdev,
    },
};

pub static BBW_MULT: f64 = 2.0;

pub struct BollingerBandsWidthConfig {
    pub length: usize,
    pub src: AnySrc,
    pub mult: f64,
}

impl IncrementalDefault for BollingerBandsWidthConfig {
    fn default(ctx: Context) -> Self {
        Self {
            length: 20,
            src: Box::new(Src::new(ctx.clone(), SrcKind::Close)),
            mult: BBW_MULT,
        }
    }
}

/// Ported from https://www.tradingview.com/chart/?solution=43000501972
pub struct BollingerBandsWidth {
    pub config: BollingerBandsWidthConfig,
    pub ctx: Context,
    basis: Sma,
    stdev: Stdev,
}

impl BollingerBandsWidth {
    pub fn new(ctx: Context, config: BollingerBandsWidthConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            basis: Sma::new(ctx.clone(), config.length),
            stdev: Stdev::new(ctx.clone(), config.length, true),
            config,
        };
    }
}

impl Incremental<(), Option<f64>> for BollingerBandsWidth {
    fn next(&mut self, _: ()) -> Option<f64> {
        let src = self.config.src.next(());
        let basis = self.basis.next(src);
        let dev = self.stdev.next(src);

        if src.is_none() || basis.is_none() || dev.is_none() {
            return None;
        }

        let basis = basis.unwrap();

        if basis == 0.0 {
            return None;
        }

        let dev = dev.unwrap() * self.config.mult;
        let upper = basis + dev;
        let lower = basis - dev;
        let bbw = (upper - lower) / basis;

        return Some(bbw);
    }
}
