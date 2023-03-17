use crate::{
    components::{component::Component, component_context::ComponentContext},
    pinescript::common::{ps_diff, ps_div, ps_max, ps_min},
    ta::sum_component::SumComponent,
};

pub static UO_MIN_VALUE: f64 = 0.0;
pub static UO_MAX_VALUE: f64 = 100.0;

pub struct UoIndicatorConfig {
    pub short_length: usize,
    pub mid_length: usize,
    pub long_length: usize,
}

impl Default for UoIndicatorConfig {
    fn default() -> Self {
        Self {
            short_length: 7,
            mid_length: 14,
            long_length: 28,
        }
    }
}

/// Ultimate Oscillator Indicator.
///
/// Ported from https://www.tradingview.com/chart/?solution=43000502328
pub struct UoIndicator {
    pub config: UoIndicatorConfig,
    pub ctx: ComponentContext,
    short_sum_bp: SumComponent,
    short_sum_tr: SumComponent,
    mid_sum_bp: SumComponent,
    mid_sum_tr: SumComponent,
    long_sum_bp: SumComponent,
    long_sum_tr: SumComponent,
}

impl UoIndicator {
    pub fn new(ctx: ComponentContext, config: UoIndicatorConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            short_sum_bp: SumComponent::new(ctx.clone(), config.short_length),
            short_sum_tr: SumComponent::new(ctx.clone(), config.short_length),
            mid_sum_bp: SumComponent::new(ctx.clone(), config.mid_length),
            mid_sum_tr: SumComponent::new(ctx.clone(), config.mid_length),
            long_sum_bp: SumComponent::new(ctx.clone(), config.long_length),
            long_sum_tr: SumComponent::new(ctx.clone(), config.long_length),
            config,
        };
    }
}

impl Component<(), Option<f64>> for UoIndicator {
    fn next(&mut self, _: ()) -> Option<f64> {
        let high = self.ctx.high();
        let low = self.ctx.low();
        let close = self.ctx.close();
        let current_tick = self.ctx.bar_index();

        let prev_close = if current_tick > 0 {
            self.ctx.prev_close(1)
        } else {
            None
        };

        let high_ = ps_max(high, prev_close);
        let low_ = ps_min(low, prev_close);
        let bp = ps_diff(close, low_);
        let tr_ = ps_diff(high_, low_);

        let fast_bp_sum = self.short_sum_bp.next(bp);
        let fast_tr_sum = self.short_sum_tr.next(tr_);

        let mid_bp_sum = self.mid_sum_bp.next(bp);
        let mid_tr_sum = self.mid_sum_tr.next(tr_);

        let slow_bp_sum = self.long_sum_bp.next(bp);
        let slow_tr_sum = self.long_sum_tr.next(tr_);

        let fast = ps_div(fast_bp_sum, fast_tr_sum);
        let mid = ps_div(mid_bp_sum, mid_tr_sum);
        let slow = ps_div(slow_bp_sum, slow_tr_sum);

        let uo = match (fast, mid, slow) {
            (Some(fast), Some(mid), Some(slow)) => {
                Some(100.0 * (4.0 * fast + 2.0 * mid + slow) / 7.0)
            }
            _ => None,
        };

        return uo;
    }
}
