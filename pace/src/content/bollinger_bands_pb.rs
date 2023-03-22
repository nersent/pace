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

pub static BOLLINGER_BANDS_PERCENT_B_MULT: f64 = 2.0;

pub struct BollingerBandsPercentBConfig {
    pub length: usize,
    pub src: AnySrc,
    pub mult: f64,
}

impl IncrementalDefault for BollingerBandsPercentBConfig {
    fn default(ctx: Context) -> Self {
        Self {
            length: 20,
            src: Box::new(Src::new(ctx.clone(), SrcKind::Close)),
            mult: BOLLINGER_BANDS_PERCENT_B_MULT,
        }
    }
}

/// Ported from https://www.tradingview.com/chart/?solution=43000501971
pub struct BollingerBandsPercentB {
    pub config: BollingerBandsPercentBConfig,
    pub ctx: Context,
    basis: Sma,
    stdev: Stdev,
}

impl BollingerBandsPercentB {
    pub fn new(ctx: Context, config: BollingerBandsPercentBConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            basis: Sma::new(ctx.clone(), config.length),
            stdev: Stdev::new(ctx.clone(), config.length, true),
            config,
        };
    }
}

impl Incremental<(), Option<f64>> for BollingerBandsPercentB {
    fn next(&mut self, _: ()) -> Option<f64> {
        let src = self.config.src.next(());
        let basis = self.basis.next(src);
        let dev = self.stdev.next(src);

        if src.is_none() || basis.is_none() || dev.is_none() {
            return None;
        }

        let src = src.unwrap();
        let basis = basis.unwrap();
        let dev = dev.unwrap() * self.config.mult;
        let upper = basis + dev;
        let lower = basis - dev;
        let upper_lower_diff = upper - lower;

        if upper_lower_diff == 0.0 {
            return None;
        }

        let bbr = (src - lower) / upper_lower_diff;

        return Some(bbr);
    }
}

pub static BOLLINGER_BANDS_PERCENT_B_THRESHOLD_OVERSOLD: f64 = 0.0;
pub static BOLLINGER_BANDS_PERCENT_B_THRESHOLD_OVERBOUGHT: f64 = 1.0;

pub struct BollingerBandsPercentBStrategyConfig {
    pub threshold_oversold: f64,
    pub threshold_overbought: f64,
}

impl Default for BollingerBandsPercentBStrategyConfig {
    fn default() -> Self {
        return Self {
            threshold_overbought: BOLLINGER_BANDS_PERCENT_B_THRESHOLD_OVERBOUGHT,
            threshold_oversold: BOLLINGER_BANDS_PERCENT_B_THRESHOLD_OVERSOLD,
        };
    }
}

/// Custom Bollinger Bands %B Strategy. May be incorrect.
///
/// Ported from https://www.tradingview.com/chart/?solution=43000589104
pub struct BollingerBandsPercentBStrategy {
    pub config: BollingerBandsPercentBStrategyConfig,
    pub ctx: Context,
    cross_over: CrossOverThreshold,
    cross_under: CrossUnderThreshold,
}

impl BollingerBandsPercentBStrategy {
    pub fn new(ctx: Context, config: BollingerBandsPercentBStrategyConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            cross_over: CrossOverThreshold::new(ctx.clone(), config.threshold_oversold),
            cross_under: CrossUnderThreshold::new(ctx.clone(), config.threshold_overbought),
            config,
        };
    }
}

impl Incremental<Option<f64>, Option<TradeDirection>> for BollingerBandsPercentBStrategy {
    fn next(&mut self, bbpb: Option<f64>) -> Option<TradeDirection> {
        let is_cross_over = self.cross_over.next(bbpb);
        let is_cross_under = self.cross_under.next(bbpb);

        let result = if is_cross_over {
            Some(TradeDirection::Long)
        } else if is_cross_under {
            Some(TradeDirection::Short)
        } else {
            None
        };

        return result;
    }
}
