use crate::{
    components::{
        component::Component, component_context::ComponentContext,
        component_default::ComponentDefault,
    },
    pinescript::common::ps_div,
    ta::{sum_component::SumComponent, swma_component::SwmaComponent},
};

pub static RVGI_MIN_VALUE: f64 = -1.0;
pub static RVGI_MAX_VALUE: f64 = 1.0;

pub struct RvgiIndicatorConfig {
    pub length: usize,
}

impl ComponentDefault for RvgiIndicatorConfig {
    fn default(ctx: ComponentContext) -> Self {
        Self { length: 10 }
    }
}

pub struct RvgiIndicatorData {
    pub rvi: Option<f64>,
    pub sig: Option<f64>,
}

/// Relative Vigor Index Indicator.
///
/// Ported from https://www.tradingview.com/chart/?solution=43000591593
pub struct RvgiIndicator {
    pub config: RvgiIndicatorConfig,
    pub ctx: ComponentContext,
    swma_close_open: SwmaComponent,
    swma_high_low: SwmaComponent,
    sum_close_open: SumComponent,
    sum_high_low: SumComponent,
    swma_sig: SwmaComponent,
}

impl RvgiIndicator {
    pub fn new(ctx: ComponentContext, config: RvgiIndicatorConfig) -> Self {
        return RvgiIndicator {
            ctx: ctx.clone(),
            swma_close_open: SwmaComponent::new(ctx.clone()),
            swma_high_low: SwmaComponent::new(ctx.clone()),
            sum_close_open: SumComponent::new(ctx.clone(), config.length),
            sum_high_low: SumComponent::new(ctx.clone(), config.length),
            swma_sig: SwmaComponent::new(ctx.clone()),
            config,
        };
    }
}

impl Component<(), RvgiIndicatorData> for RvgiIndicator {
    fn next(&mut self, _: ()) -> RvgiIndicatorData {
        let close = self.ctx.close();
        let open = self.ctx.open();
        let high = self.ctx.high();
        let low = self.ctx.low();

        let close_open_diff = match (close, open) {
            (Some(close), Some(open)) => Some(close - open),
            _ => None,
        };

        let high_low_diff = match (high, low) {
            (Some(high), Some(low)) => Some(high - low),
            _ => None,
        };

        let close_open_swma = self.swma_close_open.next(close_open_diff);
        let high_low_swma = self.swma_high_low.next(high_low_diff);

        let close_open_sum = self.sum_close_open.next(close_open_swma);
        let high_low_sum = self.sum_high_low.next(high_low_swma);

        let rvi = ps_div(close_open_sum, high_low_sum);

        let sig = self.swma_sig.next(rvi);

        return RvgiIndicatorData { rvi, sig };
    }
}
