use crate::{
    common::src::{AnySrc, Src, SrcKind},
    core::{
        context::Context,
        incremental::{Incremental, IncrementalDefault},
    },
    pinescript::common::{ps_diff, ps_div},
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

pub static VOLUME_OSCILLATOR_MIN_VALUE: f64 = -100.0;
pub static VOLUME_OSCILLATOR_MAX_VALUE: f64 = 100.0;

pub struct VolumeOscillatorConfig {
    pub short_ma: AnyMa,
    pub long_ma: AnyMa,
}

impl IncrementalDefault for VolumeOscillatorConfig {
    fn default(ctx: Context) -> Self {
        Self {
            short_ma: Ma::new(ctx.clone(), MaKind::EMA, 5).to_box(),
            long_ma: Ma::new(ctx.clone(), MaKind::EMA, 10).to_box(),
        }
    }
}

/// Ported from https://www.tradingview.com/chart/?solution=43000591350
pub struct VolumeOscillator {
    pub config: VolumeOscillatorConfig,
    pub ctx: Context,
}

impl VolumeOscillator {
    pub fn new(ctx: Context, config: VolumeOscillatorConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            config,
        };
    }
}

impl Incremental<(), Option<f64>> for VolumeOscillator {
    fn next(&mut self, _: ()) -> Option<f64> {
        let volume = self.ctx.bar.volume();

        let short_ma = self.config.short_ma.next(volume);
        let long_ma = self.config.long_ma.next(volume);

        let osc = ps_div(ps_diff(short_ma, long_ma), long_ma).map(|x| x * 100.0);

        return osc;
    }
}

pub static VOLUME_OSCILLATOR_THRESHOLD_OVERSOLD: f64 = 0.0;
pub static VOLUME_OSCILLATOR_THRESHOLD_OVERBOUGHT: f64 = 0.0;

pub struct VolumeOscillatorStrategyConfig {
    pub threshold_oversold: f64,
    pub threshold_overbought: f64,
}

impl Default for VolumeOscillatorStrategyConfig {
    fn default() -> Self {
        return Self {
            threshold_oversold: VOLUME_OSCILLATOR_THRESHOLD_OVERSOLD,
            threshold_overbought: VOLUME_OSCILLATOR_THRESHOLD_OVERBOUGHT,
        };
    }
}

/// Custom Volume Oscillator Strategy. May be incorrect.
pub struct VolumeOscillatorStrategy {
    pub ctx: Context,
    cross_over: CrossOverThreshold,
    cross_under: CrossUnderThreshold,
}

impl VolumeOscillatorStrategy {
    pub fn new(ctx: Context, config: VolumeOscillatorStrategyConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            cross_over: CrossOverThreshold::new(ctx.clone(), config.threshold_oversold),
            cross_under: CrossUnderThreshold::new(ctx.clone(), config.threshold_overbought),
        };
    }
}

impl Incremental<Option<f64>, Option<TradeDirection>> for VolumeOscillatorStrategy {
    fn next(&mut self, vo: Option<f64>) -> Option<TradeDirection> {
        let is_cross_over = self.cross_over.next(vo);
        let is_cross_under = self.cross_under.next(vo);

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
