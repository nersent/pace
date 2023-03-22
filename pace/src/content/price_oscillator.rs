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
    },
};

pub struct PriceOscillatorConfig {
    pub src: AnySrc,
    pub short_ma: AnyMa,
    pub long_ma: AnyMa,
}

impl IncrementalDefault for PriceOscillatorConfig {
    fn default(ctx: Context) -> Self {
        Self {
            src: Src::new(ctx.clone(), SrcKind::Close).to_box(),
            long_ma: Ma::new(ctx.clone(), MaKind::SMA, 10).to_box(),
            short_ma: Ma::new(ctx.clone(), MaKind::SMA, 21).to_box(),
        }
    }
}

/// Ported from https://www.tradingview.com/chart/?solution=43000502346
pub struct PriceOscillator {
    pub config: PriceOscillatorConfig,
    pub ctx: Context,
}

impl PriceOscillator {
    pub fn new(ctx: Context, config: PriceOscillatorConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            config,
        };
    }
}

impl Incremental<(), Option<f64>> for PriceOscillator {
    fn next(&mut self, _: ()) -> Option<f64> {
        let src = self.config.src.next(());

        let short_ma = self.config.short_ma.next(src);
        let long_ma = self.config.long_ma.next(src);

        let po: Option<f64> = match (short_ma, long_ma) {
            (Some(short_ma), Some(long_ma)) => Some((short_ma - long_ma) / long_ma * 100.0),
            _ => None,
        };

        return po;
    }
}

pub static PO_THRESHOLD_OVERSOLD: f64 = 0.0;
pub static PO_THRESHOLD_OVERBOUGHT: f64 = 0.0;

pub struct PriceOscillatorStrategyConfig {
    pub threshold_oversold: f64,
    pub threshold_overbought: f64,
}

impl Default for PriceOscillatorStrategyConfig {
    fn default() -> Self {
        return Self {
            threshold_oversold: PO_THRESHOLD_OVERSOLD,
            threshold_overbought: PO_THRESHOLD_OVERBOUGHT,
        };
    }
}

/// Custom Price Oscillator Strategy. May be incorrect.
pub struct PriceOscillatorStrategy {
    pub config: PriceOscillatorStrategyConfig,
    pub ctx: Context,
    cross_over: CrossOverThreshold,
    cross_under: CrossUnderThreshold,
}

impl PriceOscillatorStrategy {
    pub fn new(ctx: Context, config: PriceOscillatorStrategyConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            cross_over: CrossOverThreshold::new(ctx.clone(), config.threshold_oversold),
            cross_under: CrossUnderThreshold::new(ctx.clone(), config.threshold_overbought),
            config,
        };
    }
}

impl Incremental<Option<f64>, Option<TradeDirection>> for PriceOscillatorStrategy {
    fn next(&mut self, po: Option<f64>) -> Option<TradeDirection> {
        let is_cross_over = self.cross_over.next(po);
        let is_cross_under = self.cross_under.next(po);

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
