use crate::{
    components::{component::Component, component_context::ComponentContext},
    pinescript::math::ps_div,
    ta::sum_component::SumComponent,
};

pub struct CmfIndicatorConfig {
    pub length: usize,
}

impl Default for CmfIndicatorConfig {
    fn default() -> Self {
        Self { length: 20 }
    }
}

pub struct CmfIndicator {
    pub config: CmfIndicatorConfig,
    pub ctx: ComponentContext,
    volume_sum: SumComponent,
    ad_sum: SumComponent,
}

impl CmfIndicator {
    pub fn new(ctx: ComponentContext, config: CmfIndicatorConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            volume_sum: SumComponent::new(ctx.clone(), config.length),
            ad_sum: SumComponent::new(ctx.clone(), config.length),
            config,
        };
    }
}

impl Component<(), Option<f64>> for CmfIndicator {
    fn next(&mut self, _: ()) -> Option<f64> {
        let close = self.ctx.close();
        let high = self.ctx.high();
        let low = self.ctx.low();
        let volume = self.ctx.volume();

        let volume_sum = self.volume_sum.next(volume);

        let ad: Option<f64> = match (close, high, low, volume) {
            (Some(close), Some(high), Some(low), Some(volume)) => {
                if close == high && close == low || high == low {
                    Some(0.0)
                } else {
                    Some(((2.0 * close - low - high) / (high - low)) * volume)
                }
            }
            _ => None,
        };

        let ad_sum = self.ad_sum.next(ad);

        let cmf = ps_div(ad_sum, volume_sum);

        return cmf;
    }
}
