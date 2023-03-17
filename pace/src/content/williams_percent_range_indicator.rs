use crate::{
    components::{
        component::Component,
        component_context::ComponentContext,
        component_default::ComponentDefault,
        src::SrcKind,
        src_component::{AnySrcComponent, SrcComponent},
    },
    pinescript::common::{ps_diff, ps_div},
    ta::{highest_component::HighestComponent, lowest_component::LowestComponent},
};

pub static WPR_MIN_VALUE: f64 = -100.0;
pub static WPR_MAX_VALUE: f64 = 0.0;

pub struct WprIndicatorConfig {
    pub length: usize,
    pub src: AnySrcComponent,
}

impl ComponentDefault for WprIndicatorConfig {
    fn default(ctx: ComponentContext) -> Self {
        Self {
            length: 14,
            src: SrcComponent::build(ctx.clone(), SrcKind::Close),
        }
    }
}

/// Williams %r Indicator.
///
/// Ported from https://www.tradingview.com/chart/?solution=43000501985
pub struct WprIndicator {
    pub config: WprIndicatorConfig,
    pub ctx: ComponentContext,
    highest: HighestComponent,
    lowest: LowestComponent,
}

impl WprIndicator {
    pub fn new(ctx: ComponentContext, config: WprIndicatorConfig) -> Self {
        return WprIndicator {
            ctx: ctx.clone(),
            highest: HighestComponent::new(ctx.clone(), config.length),
            lowest: LowestComponent::new(ctx.clone(), config.length),
            config,
        };
    }
}

impl Component<(), Option<f64>> for WprIndicator {
    fn next(&mut self, _: ()) -> Option<f64> {
        let src = self.config.src.next(());

        let max = self.highest.next(self.ctx.high());
        let min = self.lowest.next(self.ctx.low());

        let pr = ps_div(ps_diff(src, max), ps_diff(max, min)).map(|x| x * 100.0);

        return pr;
    }
}
