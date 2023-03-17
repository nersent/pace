use crate::{
    components::{component::Component, component_context::ComponentContext},
    pinescript::common::{ps_abs, ps_diff, ps_div},
    ta::{atr_component::AtrComponent, sum_component::SumComponent},
};

pub struct VortexIndicatorConfig {
    pub length: usize,
}

impl Default for VortexIndicatorConfig {
    fn default() -> Self {
        Self { length: 14 }
    }
}

pub struct VortexIndicatorData {
    pub plus: Option<f64>,
    pub minus: Option<f64>,
}

/// Vortex Indicator.
///
/// Ported from https://www.tradingview.com/chart/?solution=43000591352
pub struct VortexIndicator {
    pub config: VortexIndicatorConfig,
    pub ctx: ComponentContext,
    vmp_sum: SumComponent,
    vmm_sum: SumComponent,
    atr_sum: SumComponent,
    atr: AtrComponent,
}

impl VortexIndicator {
    pub fn new(ctx: ComponentContext, config: VortexIndicatorConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            vmp_sum: SumComponent::new(ctx.clone(), config.length),
            vmm_sum: SumComponent::new(ctx.clone(), config.length),
            atr_sum: SumComponent::new(ctx.clone(), config.length),
            atr: AtrComponent::new(ctx.clone(), 1),
            config,
        };
    }
}

impl Component<(), VortexIndicatorData> for VortexIndicator {
    fn next(&mut self, _: ()) -> VortexIndicatorData {
        let current_tick = self.ctx.bar_index();
        let high = self.ctx.high();
        let low = self.ctx.low();
        let prev_high = self.ctx.prev_high(1);
        let prev_low = self.ctx.prev_low(1);

        let high_prev_low_diff = ps_abs(ps_diff(high, prev_low));
        let low_prev_high_diff = ps_abs(ps_diff(low, prev_high));

        let vmp = self.vmp_sum.next(high_prev_low_diff);
        let vmm = self.vmm_sum.next(low_prev_high_diff);

        let atr = self.atr.next(());
        let str = self.atr_sum.next(atr);

        let vip = ps_div(vmp, str);
        let vim = ps_div(vmm, str);

        return VortexIndicatorData {
            plus: vip,
            minus: vim,
        };
    }
}
