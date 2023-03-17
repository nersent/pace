use super::data_source_component::AnySourceComponent;
use super::ma_component::AnyMaComponent;
use super::sma_component::SmaComponent;
use super::{component::Component, source_kind::SourceKind};
use crate::base::components::common::batch_validator_component::BatchValidatorComponent;
use crate::base::components::common::fixed_value_cache_component::FixedValueCacheComponent;
use crate::base::components::component_context::ComponentContext;

pub struct AoIndicatorConfig {
    pub short_src: AnySourceComponent,
    pub long_src: AnySourceComponent,
    pub short_ma: AnyMaComponent,
    pub long_ma: AnyMaComponent,
}

pub struct AoIndicator {
    pub config: AoIndicatorConfig,
    pub ctx: ComponentContext,

    prev_ao: Option<f64>,
}

impl AoIndicator {
    pub fn new(ctx: ComponentContext, config: AoIndicatorConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            config,
            prev_ao: None,
        };
    }
}

impl Component<(), Option<f64>> for AoIndicator {
    fn next(&mut self, _: ()) -> Option<f64> {
        let short_ma_src = self.config.short_src.next(());
        let long_ma_src = self.config.long_src.next(());

        let short_ma = self.config.short_ma.next(short_ma_src);
        let long_ma = self.config.long_ma.next(long_ma_src);

        let ao = match (short_ma, long_ma) {
            (Some(short_ma), Some(long_ma)) => Some(short_ma - long_ma),
            _ => None,
        };

        let osc = match (ao, self.prev_ao) {
            (Some(ao), Some(prev_ao)) => Some(ao - prev_ao),
            _ => None,
        };

        self.prev_ao = ao;

        return osc;
    }
}
