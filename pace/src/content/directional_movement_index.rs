use crate::{
    common::{
        fixnan::FixNan,
        src::{AnySrc, Src, SrcKind},
    },
    core::{
        context::Context,
        incremental::{Incremental, IncrementalDefault},
    },
    pinescript::common::{ps_diff, ps_div},
    strategy::trade::TradeDirection,
    ta::{
        cross::{Cross, CrossMode},
        cross_over_threshold::CrossOverThreshold,
        cross_under_threshold::CrossUnderThreshold,
        highest_bars::HighestBars,
        lowest_bars::LowestBars,
        moving_average::{AnyMa, Ma, MaKind},
        running_moving_average::Rma,
        true_range::Tr,
    },
};

pub static DIRECTIONAL_MOVEMENT_INDEX_MIN_VALUE: f64 = 0.0;
pub static DIRECTIONAL_MOVEMENT_INDEX_MAX_VALUE: f64 = 100.0;

pub struct DirectionalMovementIndexConfig {
    pub length: usize,
    pub lensig: usize,
}

impl Default for DirectionalMovementIndexConfig {
    fn default() -> Self {
        Self {
            length: 14,
            lensig: 14,
        }
    }
}

pub struct DirectionalMovementIndexData {
    pub plus: Option<f64>,
    pub minus: Option<f64>,
    pub adx: Option<f64>,
}

/// Ported from https://www.tradingview.com/chart/?solution=43000502250
pub struct DirectionalMovementIndex {
    pub config: DirectionalMovementIndexConfig,
    pub ctx: Context,
    true_range: Tr,
    true_range_rma: Rma,
    plus_dm_rma: Rma,
    minus_dm_rma: Rma,
    plus_fix_nan: FixNan,
    minus_fix_nan: FixNan,
    adx: Rma,
}

impl DirectionalMovementIndex {
    pub fn new(ctx: Context, config: DirectionalMovementIndexConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            true_range: Tr::new(ctx.clone(), false),
            true_range_rma: Rma::new(ctx.clone(), config.length),
            plus_dm_rma: Rma::new(ctx.clone(), config.length),
            minus_dm_rma: Rma::new(ctx.clone(), config.length),
            plus_fix_nan: FixNan::new(ctx.clone()),
            minus_fix_nan: FixNan::new(ctx.clone()),
            adx: Rma::new(ctx.clone(), config.lensig),
            config,
        };
    }
}

impl Incremental<(), DirectionalMovementIndexData> for DirectionalMovementIndex {
    fn next(&mut self, _: ()) -> DirectionalMovementIndexData {
        let high = self.ctx.bar.high();
        let low = self.ctx.bar.low();
        let prev_high = self.ctx.high(1);
        let prev_low = self.ctx.low(1);

        let up = ps_diff(high, prev_high);
        let down = ps_diff(prev_low, low);

        let plus_dm = match (up, down) {
            (Some(up), Some(down)) => {
                if up > down && up > 0.0 {
                    Some(up)
                } else {
                    Some(0.0)
                }
            }
            _ => None,
        };

        let minus_dm = match (up, down) {
            (Some(up), Some(down)) => {
                if down > up && down > 0.0 {
                    Some(down)
                } else {
                    Some(0.0)
                }
            }
            _ => None,
        };

        let true_range = self.true_range.next(());
        let true_range_rma = self.true_range_rma.next(true_range);

        let plus_dm_rma = self.plus_dm_rma.next(plus_dm);
        let minus_dm_rma = self.minus_dm_rma.next(minus_dm);

        let plus = ps_div(plus_dm_rma, true_range_rma).map(|x| x * 100.0);
        let minus = ps_div(minus_dm_rma, true_range_rma).map(|x| x * 100.0);

        let plus = self.plus_fix_nan.next(plus);
        let minus = self.minus_fix_nan.next(minus);

        let adx: Option<f64> = match (plus, minus) {
            (Some(plus), Some(minus)) => {
                Some((plus - minus).abs() / (if plus == -minus { 0.0 } else { plus + minus }))
            }
            _ => None,
        };
        let adx = self.adx.next(adx).map(|x| x * 100.0);

        return DirectionalMovementIndexData { plus, minus, adx };
    }
}

pub static DIRECTIONAL_MOVEMENT_INDEX_THRESHOLD_STRONG_TREND: f64 = 25.0;
pub static DIRECTIONAL_MOVEMENT_INDEX_THRESHOLD_WEAK_TREND: f64 = 20.0;

pub struct DirectionalMovementIndexStrategyConfig {
    pub threshold_strong_trend: f64,
    pub threshold_weak_trend: f64,
}

impl Default for DirectionalMovementIndexStrategyConfig {
    fn default() -> Self {
        return Self {
            threshold_strong_trend: DIRECTIONAL_MOVEMENT_INDEX_THRESHOLD_STRONG_TREND,
            threshold_weak_trend: DIRECTIONAL_MOVEMENT_INDEX_THRESHOLD_WEAK_TREND,
        };
    }
}

/// Custom Directional Movement Index Strategy. May be incorrect.
pub struct DirectionalMovementIndexStrategy {
    pub config: DirectionalMovementIndexStrategyConfig,
    pub ctx: Context,
    cross: Cross,
}

impl DirectionalMovementIndexStrategy {
    pub fn new(ctx: Context, config: DirectionalMovementIndexStrategyConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            cross: Cross::new(ctx.clone()),
            config,
        };
    }
}

impl Incremental<&DirectionalMovementIndexData, Option<TradeDirection>>
    for DirectionalMovementIndexStrategy
{
    fn next(&mut self, dmi: &DirectionalMovementIndexData) -> Option<TradeDirection> {
        let is_strong_trend = dmi
            .adx
            .map(|x| x > self.config.threshold_strong_trend)
            .unwrap_or(false);

        let is_weak_trend = dmi
            .adx
            .map(|x| x < self.config.threshold_weak_trend)
            .unwrap_or(false);

        let plus_minus_cross = self.cross.next((dmi.plus, dmi.minus));

        let mut result: Option<TradeDirection> = None;

        if is_strong_trend {
            if let Some(plus_minus_cross) = plus_minus_cross {
                result = match plus_minus_cross {
                    CrossMode::Over => Some(TradeDirection::Long),
                    CrossMode::Under => Some(TradeDirection::Short),
                }
            }
        }

        return result;
    }
}
