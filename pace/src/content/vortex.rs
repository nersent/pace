use crate::{
    common::src::{AnySrc, Src, SrcKind},
    core::{
        context::Context,
        incremental::{Incremental, IncrementalDefault},
    },
    pinescript::common::{ps_abs, ps_diff, ps_div},
    strategy::trade::TradeDirection,
    ta::{
        average_true_range::Atr,
        cross::{Cross, CrossMode},
        cross_over_threshold::CrossOverThreshold,
        cross_under_threshold::CrossUnderThreshold,
        highest_bars::HighestBars,
        lowest_bars::LowestBars,
        moving_average::{AnyMa, Ma, MaKind},
        sum::Sum,
    },
};

pub struct VortexConfig {
    pub length: usize,
}

impl Default for VortexConfig {
    fn default() -> Self {
        Self { length: 14 }
    }
}

pub struct VortexData {
    pub plus: Option<f64>,
    pub minus: Option<f64>,
}

/// Ported from https://www.tradingview.com/chart/?solution=43000591352
pub struct Vortex {
    pub config: VortexConfig,
    pub ctx: Context,
    vmp_sum: Sum,
    vmm_sum: Sum,
    atr_sum: Sum,
    atr: Atr,
}

impl Vortex {
    pub fn new(ctx: Context, config: VortexConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            vmp_sum: Sum::new(ctx.clone(), config.length),
            vmm_sum: Sum::new(ctx.clone(), config.length),
            atr_sum: Sum::new(ctx.clone(), config.length),
            atr: Atr::new(ctx.clone(), 1),
            config,
        };
    }
}

impl Incremental<(), VortexData> for Vortex {
    fn next(&mut self, _: ()) -> VortexData {
        let current_tick = self.ctx.bar.index();
        let high = self.ctx.bar.high();
        let low = self.ctx.bar.low();
        let prev_high = self.ctx.high(1);
        let prev_low = self.ctx.low(1);

        let high_prev_low_diff = ps_abs(ps_diff(high, prev_low));
        let low_prev_high_diff = ps_abs(ps_diff(low, prev_high));

        let vmp = self.vmp_sum.next(high_prev_low_diff);
        let vmm = self.vmm_sum.next(low_prev_high_diff);

        let atr = self.atr.next(());
        let str = self.atr_sum.next(atr);

        let vip = ps_div(vmp, str);
        let vim = ps_div(vmm, str);

        return VortexData {
            plus: vip,
            minus: vim,
        };
    }
}

pub struct VortexStrategy {
    pub ctx: Context,
    cross: Cross,
}

/// Custom Vortex Strategy. May be incorrect.
impl VortexStrategy {
    pub fn new(ctx: Context) -> Self {
        return Self {
            ctx: ctx.clone(),
            cross: Cross::new(ctx.clone()),
        };
    }
}

impl Incremental<&VortexData, Option<TradeDirection>> for VortexStrategy {
    fn next(&mut self, vi: &VortexData) -> Option<TradeDirection> {
        let vip_vim_cross = self.cross.next((vi.plus, vi.minus));

        let mut result: Option<TradeDirection> = None;

        if let Some(plus_minus_cross) = vip_vim_cross {
            result = match plus_minus_cross {
                CrossMode::Over => Some(TradeDirection::Long),
                CrossMode::Under => Some(TradeDirection::Short),
            }
        }

        return result;
    }
}
