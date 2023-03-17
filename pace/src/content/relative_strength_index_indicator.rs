use crate::{
    components::{
        component::Component,
        component_context::ComponentContext,
        component_default::ComponentDefault,
        src::SrcKind,
        src_component::{AnySrcComponent, SrcComponent},
    },
    ta::rsi_component::RsiComponent,
};

pub struct RsiIndicatorConfig {
    pub length: usize,
    pub src: AnySrcComponent,
}

impl ComponentDefault for RsiIndicatorConfig {
    fn default(ctx: ComponentContext) -> Self {
        return Self {
            length: 14,
            src: SrcComponent::build(ctx.clone(), SrcKind::Close),
        };
    }
}

/// Relative Strength Index Indicator.
///
/// Ported from https://www.tradingview.com/chart/?solution=43000502338
pub struct RsiIndicator {
    pub ctx: ComponentContext,
    pub config: RsiIndicatorConfig,
    rsi: RsiComponent,
}

impl RsiIndicator {
    pub fn new(ctx: ComponentContext, config: RsiIndicatorConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            rsi: RsiComponent::new(ctx.clone(), config.length),
            config,
        };
    }
}

impl Component<(), Option<f64>> for RsiIndicator {
    fn next(&mut self, _: ()) -> Option<f64> {
        let src = self.config.src.next(());
        return self.rsi.next(src);
    }
}
