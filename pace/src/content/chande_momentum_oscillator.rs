use crate::{
    common::src::{AnySrc, Src, SrcKind},
    core::{
        context::Context,
        incremental::{Incremental, IncrementalDefault},
    },
    pinescript::common::{ps_abs, ps_diff, ps_max, ps_min},
    strategy::trade::TradeDirection,
    ta::{
        cross::Cross,
        cross_over_threshold::CrossOverThreshold,
        cross_under_threshold::CrossUnderThreshold,
        highest_bars::HighestBars,
        lowest_bars::LowestBars,
        moving_average::{AnyMa, Ma, MaKind},
        sum::Sum,
    },
};

pub static CHANDE_MOMENTUM_OSCILLATOR_MAX_VALUE: f64 = 100.0;

pub struct ChandeMomentumOscillatorConfig {
    pub length: usize,
    pub src: AnySrc,
}

impl IncrementalDefault for ChandeMomentumOscillatorConfig {
    fn default(ctx: Context) -> Self {
        Self {
            length: 9,
            src: Box::new(Src::new(ctx.clone(), SrcKind::Close)),
        }
    }
}

/// Ported from https://www.tradingview.com/chart/?solution=43000589109
pub struct ChandeMomentumOscillator {
    pub config: ChandeMomentumOscillatorConfig,
    pub ctx: Context,
    prev_src: Option<f64>,
    sm1: Sum,
    sm2: Sum,
}

impl ChandeMomentumOscillator {
    pub fn new(ctx: Context, config: ChandeMomentumOscillatorConfig) -> Self {
        assert!(
            config.length > 1,
            "ChandeMomentumOscillatorIndicator length must be greater than 1"
        );
        return Self {
            ctx: ctx.clone(),
            prev_src: None,
            sm1: Sum::new(ctx.clone(), config.length),
            sm2: Sum::new(ctx.clone(), config.length),
            config,
        };
    }
}

impl Incremental<(), Option<f64>> for ChandeMomentumOscillator {
    fn next(&mut self, _: ()) -> Option<f64> {
        let src = self.config.src.next(());
        let momm = ps_diff(src, self.prev_src);

        let m1 = ps_max(Some(0.0), momm);
        let m2 = ps_abs(ps_min(Some(0.0), momm));

        let sm1 = self.sm1.next(m1);
        let sm2 = self.sm2.next(m2);

        let chande_mo: Option<f64> = match (sm1, sm2) {
            (Some(sm1), Some(sm2)) => {
                if sm1 == -sm2 {
                    None
                } else {
                    Some(100.0 * (sm1 - sm2) / (sm1 + sm2))
                }
            }
            _ => None,
        };

        self.prev_src = src;

        return chande_mo;
    }
}

pub static CHANDE_MOMENTUM_OSCILLATOR_THRESHOLD_OVERSOLD: f64 = -50.0;
pub static CHANDE_MOMENTUM_OSCILLATOR_THRESHOLD_OVERBOUGHT: f64 = 50.0;

pub struct ChandeMomentumOscillatorStrategyConfig {
    pub threshold_oversold: f64,
    pub threshold_overbought: f64,
}

impl Default for ChandeMomentumOscillatorStrategyConfig {
    fn default() -> Self {
        return Self {
            threshold_oversold: CHANDE_MOMENTUM_OSCILLATOR_THRESHOLD_OVERSOLD,
            threshold_overbought: CHANDE_MOMENTUM_OSCILLATOR_THRESHOLD_OVERBOUGHT,
        };
    }
}

/// Custom Chande Momentum Oscillator Strategy. May be incorrect.
pub struct ChandeMomentumOscillatorStrategy {
    pub config: ChandeMomentumOscillatorStrategyConfig,
    pub ctx: Context,
    cross_over: CrossOverThreshold,
    cross_under: CrossUnderThreshold,
}

impl ChandeMomentumOscillatorStrategy {
    pub fn new(ctx: Context, config: ChandeMomentumOscillatorStrategyConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            cross_over: CrossOverThreshold::new(ctx.clone(), config.threshold_oversold),
            cross_under: CrossUnderThreshold::new(ctx.clone(), config.threshold_overbought),
            config,
        };
    }
}

impl Incremental<Option<f64>, Option<TradeDirection>> for ChandeMomentumOscillatorStrategy {
    fn next(&mut self, cmf: Option<f64>) -> Option<TradeDirection> {
        let is_cross_over = self.cross_over.next(cmf);
        let is_cross_under = self.cross_under.next(cmf);

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
