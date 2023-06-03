use std::collections::HashMap;

use crate::{
    common::src::{AnySrc, Src, SrcKind},
    core::{
        context::Context,
        features::{FeatureValue, Features, IncrementalFeatureBuilder},
        incremental::{Incremental, IncrementalDefault},
        trend::Trend,
    },
    strategy::trade::{StrategySignal, TradeDirection},
    ta::{
        cross::Cross, cross_over_threshold::CrossOverThreshold,
        cross_under_threshold::CrossUnderThreshold, highest_bars::HighestBars,
        lowest_bars::LowestBars, rate_of_change::Roc, weighted_moving_average::Wma,
    },
};

pub struct CoppockCurveConfig {
    pub src: AnySrc,
    pub long_roc_length: usize,
    pub short_roc_length: usize,
    pub length: usize,
}

impl IncrementalDefault for CoppockCurveConfig {
    fn default(ctx: Context) -> Self {
        Self {
            length: 10,
            long_roc_length: 14,
            short_roc_length: 11,
            src: Src::new(ctx.clone(), SrcKind::Close).to_box(),
        }
    }
}

/// Ported from https://www.tradingview.com/chart/?solution=43000589114
pub struct CoppockCurve {
    pub config: CoppockCurveConfig,
    pub ctx: Context,
    ma: Wma,
    long_roc: Roc,
    short_roc: Roc,
}

impl CoppockCurve {
    pub fn new(ctx: Context, config: CoppockCurveConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            ma: Wma::new(ctx.clone(), config.length),
            long_roc: Roc::new(ctx.clone(), config.long_roc_length),
            short_roc: Roc::new(ctx.clone(), config.short_roc_length),
            config,
        };
    }
}

impl Incremental<(), f64> for CoppockCurve {
    fn next(&mut self, _: ()) -> f64 {
        let src = self.config.src.next(());

        let long_roc = self.long_roc.next(src);
        let short_roc = self.short_roc.next(src);
        let roc = long_roc + short_roc;
        let curve = self.ma.next(roc);

        return curve;
    }
}

pub static COPPOCK_CURVE_THRESHOLD_OVERSOLD: f64 = 0.0;
pub static COPPOCK_CURVE_THRESHOLD_OVERBOUGHT: f64 = 0.0;

pub struct CoppockCurveStrategyConfig {
    pub threshold_oversold: f64,
    pub threshold_overbought: f64,
}

impl Default for CoppockCurveStrategyConfig {
    fn default() -> Self {
        return Self {
            threshold_oversold: COPPOCK_CURVE_THRESHOLD_OVERSOLD,
            threshold_overbought: COPPOCK_CURVE_THRESHOLD_OVERBOUGHT,
        };
    }
}

/// Custom Coppock Curve Strategy. May be incorrect.
pub struct CoppockCurveStrategy {
    pub config: CoppockCurveStrategyConfig,
    pub ctx: Context,
    cross_over: CrossOverThreshold,
    cross_under: CrossUnderThreshold,
}

impl CoppockCurveStrategy {
    pub fn new(ctx: Context, config: CoppockCurveStrategyConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            cross_over: CrossOverThreshold::new(ctx.clone(), config.threshold_oversold),
            cross_under: CrossUnderThreshold::new(ctx.clone(), config.threshold_overbought),
            config,
        };
    }
}

impl Incremental<f64, StrategySignal> for CoppockCurveStrategy {
    fn next(&mut self, cc: f64) -> StrategySignal {
        let is_cross_over = self.cross_over.next(cc);
        let is_cross_under = self.cross_under.next(cc);

        if is_cross_over {
            return StrategySignal::Long;
        }
        if is_cross_under {
            return StrategySignal::Short;
        }
        return StrategySignal::Hold;
    }
}
