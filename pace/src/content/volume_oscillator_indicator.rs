use crate::{
    components::{
        component::Component, component_context::ComponentContext,
        component_default::ComponentDefault,
    },
    pinescript::math::{ps_diff, ps_div},
    ta::{
        ma::MaKind,
        ma_component::{AnyMaComponent, MaComponent},
    },
};

pub static VO_MIN_VALUE: f64 = -100.0;
pub static VO_MAX_VALUE: f64 = 100.0;

pub struct VoIndicatorConfig {
    pub short_ma: AnyMaComponent,
    pub long_ma: AnyMaComponent,
}

impl ComponentDefault for VoIndicatorConfig {
    fn default(ctx: ComponentContext) -> Self {
        Self {
            short_ma: MaComponent::build(ctx.clone(), MaKind::EMA, 5),
            long_ma: MaComponent::build(ctx.clone(), MaKind::EMA, 10),
        }
    }
}

pub struct VoIndicator {
    pub config: VoIndicatorConfig,
    pub ctx: ComponentContext,
}

impl VoIndicator {
    pub fn new(ctx: ComponentContext, config: VoIndicatorConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            config,
        };
    }
}

impl Component<(), Option<f64>> for VoIndicator {
    fn next(&mut self, _: ()) -> Option<f64> {
        let volume = self.ctx.volume();

        let short_ma = self.config.short_ma.next(volume);
        let long_ma = self.config.long_ma.next(volume);

        let osc = ps_div(ps_diff(short_ma, long_ma), long_ma).map(|x| x * 100.0);

        return osc;
    }
}
