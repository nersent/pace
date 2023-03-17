use crate::{
    components::{
        component::Component,
        component_context::ComponentContext,
        component_default::ComponentDefault,
        src::SrcKind,
        src_component::{AnySrcComponent, SrcComponent},
    },
    ta::{
        ma::MaKind,
        ma_component::{AnyMaComponent, MaComponent},
    },
};

pub struct PoIndicatorConfig {
    pub src: AnySrcComponent,
    pub short_ma: AnyMaComponent,
    pub long_ma: AnyMaComponent,
}

impl ComponentDefault for PoIndicatorConfig {
    fn default(ctx: ComponentContext) -> Self {
        Self {
            src: SrcComponent::build(ctx.clone(), SrcKind::Close),
            long_ma: MaComponent::build(ctx.clone(), MaKind::SMA, 10),
            short_ma: MaComponent::build(ctx.clone(), MaKind::SMA, 21),
        }
    }
}

/// Price Oscillator Indicator.
///
/// Ported from https://www.tradingview.com/chart/?solution=43000502346
pub struct PoIndicator {
    pub config: PoIndicatorConfig,
    pub ctx: ComponentContext,
}

impl PoIndicator {
    pub fn new(ctx: ComponentContext, config: PoIndicatorConfig) -> Self {
        return PoIndicator {
            ctx: ctx.clone(),
            config,
        };
    }
}

impl Component<(), Option<f64>> for PoIndicator {
    fn next(&mut self, _: ()) -> Option<f64> {
        let src = self.config.src.next(());

        let short_ma = self.config.short_ma.next(src);
        let long_ma = self.config.long_ma.next(src);

        let po: Option<f64> = match (short_ma, long_ma) {
            (Some(short_ma), Some(long_ma)) => Some((short_ma - long_ma) / long_ma * 100.0),
            _ => None,
        };

        return po;
    }
}
