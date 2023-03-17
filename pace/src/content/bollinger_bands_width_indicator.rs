use crate::{
    components::{
        component::Component,
        component_context::ComponentContext,
        component_default::ComponentDefault,
        src::SrcKind,
        src_component::{AnySrcComponent, SrcComponent},
    },
    ta::{sma_component::SmaComponent, stdev_component::StdevComponent},
};

pub static BBW_MULT: f64 = 2.0;

pub struct BbwIndicatorConfig {
    pub length: usize,
    pub src: AnySrcComponent,
    pub mult: f64,
}

impl ComponentDefault for BbwIndicatorConfig {
    fn default(ctx: ComponentContext) -> Self {
        Self {
            length: 20,
            src: Box::new(SrcComponent::new(ctx.clone(), SrcKind::Close)),
            mult: BBW_MULT,
        }
    }
}

/// Bollinger Bands Width Indicator.
///
/// Ported from https://www.tradingview.com/chart/?solution=43000501972
pub struct BbwIndicator {
    pub config: BbwIndicatorConfig,
    pub ctx: ComponentContext,
    basis: SmaComponent,
    stdev: StdevComponent,
}

impl BbwIndicator {
    pub fn new(ctx: ComponentContext, config: BbwIndicatorConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            basis: SmaComponent::new(ctx.clone(), config.length),
            stdev: StdevComponent::new(ctx.clone(), config.length, true),
            config,
        };
    }
}

impl Component<(), Option<f64>> for BbwIndicator {
    fn next(&mut self, _: ()) -> Option<f64> {
        let src = self.config.src.next(());
        let basis = self.basis.next(src);
        let dev = self.stdev.next(src);

        if src.is_none() || basis.is_none() || dev.is_none() {
            return None;
        }

        let basis = basis.unwrap();

        if basis == 0.0 {
            return None;
        }

        let dev = dev.unwrap() * self.config.mult;
        let upper = basis + dev;
        let lower = basis - dev;
        let bbw = (upper - lower) / basis;

        return Some(bbw);
    }
}
