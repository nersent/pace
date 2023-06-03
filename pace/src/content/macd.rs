use std::collections::HashMap;

use crate::{
    common::src::{AnyProcessor, AnySrc, Src, SrcKind},
    core::{
        context::Context,
        features::{FeatureValue, Features, IncrementalFeatureBuilder},
        incremental::{Incremental, IncrementalDefault},
        trend::Trend,
    },
    strategy::trade::{StrategySignal, TradeDirection},
    ta::{
        cross::Cross,
        cross_over_threshold::CrossOverThreshold,
        cross_under_threshold::CrossUnderThreshold,
        highest_bars::HighestBars,
        lowest_bars::LowestBars,
        moving_average::{Ma, MaKind},
    },
};

pub struct MacdConfig {
    pub short_src: AnySrc,
    pub long_src: AnySrc,
    pub short_ma: AnyProcessor,
    pub long_ma: AnyProcessor,
    pub signal_ma: AnyProcessor,
}

impl IncrementalDefault for MacdConfig {
    fn default(ctx: Context) -> Self {
        Self {
            short_ma: Ma::new(ctx.clone(), MaKind::EMA, 12).to_box(),
            long_ma: Ma::new(ctx.clone(), MaKind::EMA, 26).to_box(),
            short_src: Src::new(ctx.clone(), SrcKind::Close).to_box(),
            long_src: Src::new(ctx.clone(), SrcKind::Close).to_box(),
            signal_ma: Ma::new(ctx.clone(), MaKind::EMA, 9).to_box(),
        }
    }
}

/// Moving Average Convergence Divergence Indicator.
///
/// Ported from https://www.tradingview.com/chart/?symbol=BITSTAMP%3ABTCUSD&solution=43000502344
pub struct Macd {
    pub config: MacdConfig,
    pub ctx: Context,
}

impl Macd {
    pub fn new(ctx: Context, config: MacdConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            config,
        };
    }
}

impl Incremental<(), (f64, f64)> for Macd {
    fn next(&mut self, _: ()) -> (f64, f64) {
        let short_ma_src = self.config.short_src.next(());
        let long_ma_src = self.config.long_src.next(());

        let short_ma = self.config.short_ma.next(short_ma_src);
        let long_ma = self.config.long_ma.next(long_ma_src);

        let macd = short_ma - long_ma;

        let signal = self.config.signal_ma.next(macd);

        return (macd, signal);
    }
}

pub static MACD_THRESHOLD_OVERBOUGHT: f64 = 0.0;
pub static MACD_THRESHOLD_OVERSOLD: f64 = 0.0;

pub struct MacdStrategyConfig {
    pub threshold_overbought: f64,
    pub threshold_oversold: f64,
}

impl Default for MacdStrategyConfig {
    fn default() -> Self {
        return Self {
            threshold_overbought: MACD_THRESHOLD_OVERBOUGHT,
            threshold_oversold: MACD_THRESHOLD_OVERSOLD,
        };
    }
}

pub struct MacdStrategy {
    pub ctx: Context,
    pub config: MacdStrategyConfig,
    cross_over: CrossOverThreshold,
    cross_under: CrossUnderThreshold,
}

impl MacdStrategy {
    pub fn new(ctx: Context, config: MacdStrategyConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            cross_over: CrossOverThreshold::new(ctx.clone(), config.threshold_oversold),
            cross_under: CrossUnderThreshold::new(ctx.clone(), config.threshold_overbought),
            config,
        };
    }
}

impl Incremental<f64, StrategySignal> for MacdStrategy {
    fn next(&mut self, macd_delta: f64) -> StrategySignal {
        let is_cross_over = self.cross_over.next(macd_delta);
        let is_cross_under = self.cross_under.next(macd_delta);

        if is_cross_over {
            return StrategySignal::Long;
        }
        if is_cross_under {
            return StrategySignal::Short;
        }
        return StrategySignal::Hold;
    }
}
