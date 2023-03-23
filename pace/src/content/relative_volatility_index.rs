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
        exponential_moving_average::Ema,
        highest_bars::HighestBars,
        lowest_bars::LowestBars,
        moving_average::{AnyMa, Ma, MaKind},
        stdev::Stdev,
    },
};

pub static RELATIVE_VOLATILITY_INDEX_MIN_VALUE: f64 = 0.0;
pub static RELATIVE_VOLATILITY_INDEX_MAX_VALUE: f64 = 100.0;

pub struct RelativeVolatilityIndexConfig {
    pub length: usize,
    pub ma_length: usize,
    pub src: AnySrc,
}

impl IncrementalDefault for RelativeVolatilityIndexConfig {
    fn default(ctx: Context) -> Self {
        Self {
            length: 10,
            ma_length: 14,
            src: Src::new(ctx.clone(), SrcKind::Close).to_box(),
        }
    }
}

/// Ported from https://www.tradingview.com/chart/?solution=43000594684
pub struct RelativeVolatilityIndex {
    pub config: RelativeVolatilityIndexConfig,
    pub ctx: Context,
    stdev: Stdev,
    upper_ema: Ema,
    lower_ema: Ema,
    prev_src: Option<f64>,
}

impl RelativeVolatilityIndex {
    pub fn new(ctx: Context, config: RelativeVolatilityIndexConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            stdev: Stdev::new(ctx.clone(), config.length, true),
            upper_ema: Ema::new(ctx.clone(), config.ma_length),
            lower_ema: Ema::new(ctx.clone(), config.ma_length),
            config,
            prev_src: None,
        };
    }
}

impl Incremental<(), Option<f64>> for RelativeVolatilityIndex {
    fn next(&mut self, _: ()) -> Option<f64> {
        let src = self.config.src.next(());
        let stdev = self.stdev.next(src);
        let src_change = match (src, self.prev_src) {
            (Some(src), Some(prev_src)) => Some(src - prev_src),
            _ => None,
        };

        let (upper, lower) = match src_change {
            Some(change) => {
                let upper = if change <= 0.0 { Some(0.0) } else { stdev };
                let lower = if change > 0.0 { Some(0.0) } else { stdev };
                (upper, lower)
            }
            _ => (None, None),
        };

        let upper = self.upper_ema.next(upper);
        let lower = self.lower_ema.next(lower);

        let rvi = match (upper, lower) {
            (Some(upper), Some(lower)) => {
                if upper == -lower {
                    None
                } else {
                    Some(upper / (upper + lower) * 100.0)
                }
            }
            _ => None,
        };

        self.prev_src = src;

        return rvi;
    }
}

pub static RELATIVE_VOLATILITY_INDEX_THRESHOLD_OVERSOLD: f64 = 20.0;
pub static RELATIVE_VOLATILITY_INDEX_THRESHOLD_OVERBOUGHT: f64 = 80.0;

pub struct RelativeVolatilityIndexStrategyConfig {
    pub threshold_oversold: f64,
    pub threshold_overbought: f64,
}

impl Default for RelativeVolatilityIndexStrategyConfig {
    fn default() -> Self {
        return Self {
            threshold_oversold: RELATIVE_VOLATILITY_INDEX_THRESHOLD_OVERSOLD,
            threshold_overbought: RELATIVE_VOLATILITY_INDEX_THRESHOLD_OVERBOUGHT,
        };
    }
}

/// Custom Relative Volatility Index Strategy. May be incorrect.
pub struct RelativeVolatilityIndexStrategy {
    pub config: RelativeVolatilityIndexStrategyConfig,
    pub ctx: Context,
    cross_over: CrossOverThreshold,
    cross_under: CrossUnderThreshold,
}

impl RelativeVolatilityIndexStrategy {
    pub fn new(ctx: Context, config: RelativeVolatilityIndexStrategyConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            cross_over: CrossOverThreshold::new(ctx.clone(), config.threshold_oversold),
            cross_under: CrossUnderThreshold::new(ctx.clone(), config.threshold_overbought),
            config,
        };
    }
}

impl Incremental<Option<f64>, Option<TradeDirection>> for RelativeVolatilityIndexStrategy {
    fn next(&mut self, rsi: Option<f64>) -> Option<TradeDirection> {
        let cross_over = self.cross_over.next(rsi);
        let cross_under = self.cross_under.next(rsi);

        let result = if cross_over {
            Some(TradeDirection::Long)
        } else if cross_under {
            Some(TradeDirection::Short)
        } else {
            None
        };

        return result;
    }
}
