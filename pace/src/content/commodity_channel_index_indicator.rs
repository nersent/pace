use crate::{
    components::{
        component::Component,
        component_context::ComponentContext,
        component_default::ComponentDefault,
        src_component::{AnySrcComponent, SrcComponent},
        src_kind::SrcKind,
    },
    pinescript::common::{ps_diff, ps_div},
    ta::{
        dev_component::DevComponent,
        ma::MaKind,
        ma_component::{AnyMaComponent, MaComponent},
        sma_component::SmaComponent,
    },
};

pub struct CciIndicatorConfig {
    pub length: usize,
    pub src: AnySrcComponent,
}

impl ComponentDefault for CciIndicatorConfig {
    fn default(ctx: ComponentContext) -> Self {
        Self {
            length: 20,
            src: SrcComponent::build(ctx.clone(), SrcKind::HLC3),
        }
    }
}

pub struct CciIndicator {
    pub config: CciIndicatorConfig,
    pub ctx: ComponentContext,
    sma: SmaComponent,
    dev: DevComponent,
}

impl CciIndicator {
    pub fn new(ctx: ComponentContext, config: CciIndicatorConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            sma: SmaComponent::new(ctx.clone(), config.length),
            dev: DevComponent::new(ctx.clone(), config.length),
            config,
        };
    }
}

impl Component<(), Option<f64>> for CciIndicator {
    fn next(&mut self, _: ()) -> Option<f64> {
        let src = self.config.src.next(());
        let ma = self.sma.next(src);
        let dev = self.dev.next(src);

        let cci = ps_div(ps_diff(src, ma), dev.map(|x| x * 0.015));

        return cci;
    }
}
