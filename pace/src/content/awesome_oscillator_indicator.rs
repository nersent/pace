use crate::{
    components::{
        component::Component,
        component_context::ComponentContext,
        component_default::ComponentDefault,
        src_component::{AnySrcComponent, SrcComponent},
        src_kind::SrcKind,
    },
    ta::{
        ma::MaKind,
        ma_component::{AnyMaComponent, MaComponent},
    },
};

pub struct AoIndicatorConfig {
    pub short_src: AnySrcComponent,
    pub long_src: AnySrcComponent,
    pub short_ma: AnyMaComponent,
    pub long_ma: AnyMaComponent,
}

impl ComponentDefault for AoIndicatorConfig {
    fn default(ctx: ComponentContext) -> Self {
        Self {
            long_ma: MaComponent::build(ctx.clone(), MaKind::SMA, 34),
            short_ma: MaComponent::build(ctx.clone(), MaKind::SMA, 5),
            long_src: SrcComponent::build(ctx.clone(), SrcKind::HL2),
            short_src: SrcComponent::build(ctx.clone(), SrcKind::HL2),
        }
    }
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
