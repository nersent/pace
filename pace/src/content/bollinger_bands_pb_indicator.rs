use crate::{
    components::{
        component::Component,
        component_context::ComponentContext,
        component_default::ComponentDefault,
        src_component::{AnySrcComponent, SrcComponent},
        src_kind::SrcKind,
    },
    ta::{sma_component::SmaComponent, stdev_component::StdevComponent},
};

pub static BBPB_MULT: f64 = 2.0;

pub struct BbpbIndicatorConfig {
    pub length: usize,
    pub src: AnySrcComponent,
    pub mult: f64,
}

impl ComponentDefault for BbpbIndicatorConfig {
    fn default(ctx: ComponentContext) -> Self {
        Self {
            length: 20,
            src: Box::new(SrcComponent::new(ctx.clone(), SrcKind::Close)),
            mult: BBPB_MULT,
        }
    }
}

pub struct BbpbIndicator {
    pub config: BbpbIndicatorConfig,
    pub ctx: ComponentContext,
    basis: SmaComponent,
    stdev: StdevComponent,
}

impl BbpbIndicator {
    pub fn new(ctx: ComponentContext, config: BbpbIndicatorConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            basis: SmaComponent::new(ctx.clone(), config.length),
            stdev: StdevComponent::new(ctx.clone(), config.length, true),
            config,
        };
    }
}

impl Component<(), Option<f64>> for BbpbIndicator {
    fn next(&mut self, _: ()) -> Option<f64> {
        let src = self.config.src.next(());
        let basis = self.basis.next(src);
        let dev = self.stdev.next(src);

        if src.is_none() || basis.is_none() || dev.is_none() {
            return None;
        }

        let src = src.unwrap();
        let basis = basis.unwrap();
        let dev = dev.unwrap() * self.config.mult;
        let upper = basis + dev;
        let lower = basis - dev;
        let upper_lower_diff = upper - lower;

        if upper_lower_diff == 0.0 {
            return None;
        }

        let bbr = (src - lower) / upper_lower_diff;

        return Some(bbr);
    }
}
