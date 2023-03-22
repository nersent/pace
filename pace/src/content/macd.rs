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

pub struct MacdConfig {
    pub short_src: AnySrc,
    pub long_src: AnySrc,
    pub short_ma: AnyMa,
    pub long_ma: AnyMa,
    pub signal_ma: AnyMa,
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

impl Incremental<(), (Option<f64>, Option<f64>)> for Macd {
    fn next(&mut self, _: ()) -> (Option<f64>, Option<f64>) {
        let short_ma_src = self.config.short_src.next(());
        let long_ma_src = self.config.long_src.next(());

        let short_ma = self.config.short_ma.next(short_ma_src);
        let long_ma = self.config.long_ma.next(long_ma_src);

        let macd = match (short_ma, long_ma) {
            (Some(short_ma), Some(long_ma)) => Some(short_ma - long_ma),
            _ => None,
        };

        let signal = self.config.signal_ma.next(macd);

        return (macd, signal);
    }
}

pub struct MacdStrategy {
    pub ctx: Context,
    cross_over: CrossOverThreshold,
    cross_under: CrossUnderThreshold,
}

impl MacdStrategy {
    pub fn new(ctx: Context) -> Self {
        return Self {
            ctx: ctx.clone(),
            cross_over: CrossOverThreshold::new(ctx.clone(), 0.0),
            cross_under: CrossUnderThreshold::new(ctx.clone(), 0.0),
        };
    }
}

impl Incremental<Option<f64>, Option<TradeDirection>> for MacdStrategy {
    fn next(&mut self, macd_delta: Option<f64>) -> Option<TradeDirection> {
        let cross_over = self.cross_over.next(macd_delta);
        let cross_under = self.cross_under.next(macd_delta);

        if cross_over {
            return Some(TradeDirection::Long);
        } else if cross_under {
            return Some(TradeDirection::Short);
        }

        return None;
    }
}
