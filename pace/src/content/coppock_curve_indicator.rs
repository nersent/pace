use crate::{
    components::{
        component::Component,
        component_context::ComponentContext,
        component_default::ComponentDefault,
        src::SrcKind,
        src_component::{AnySrcComponent, SrcComponent},
    },
    ta::{roc_component::RocComponent, wma_component::WmaComponent},
};

pub struct CcIndicatorConfig {
    pub src: AnySrcComponent,
    pub long_roc_length: usize,
    pub short_roc_length: usize,
    pub ma_length: usize,
}

impl ComponentDefault for CcIndicatorConfig {
    fn default(ctx: ComponentContext) -> Self {
        Self {
            ma_length: 10,
            long_roc_length: 14,
            short_roc_length: 11,
            src: SrcComponent::build(ctx.clone(), SrcKind::Close),
        }
    }
}

/// Coppock Curve Indicator.
///
/// Ported from https://www.tradingview.com/chart/?solution=43000589114
pub struct CcIndicator {
    pub config: CcIndicatorConfig,
    pub ctx: ComponentContext,
    ma: WmaComponent,
    long_roc: RocComponent,
    short_roc: RocComponent,
}

impl CcIndicator {
    pub fn new(ctx: ComponentContext, config: CcIndicatorConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            ma: WmaComponent::new(ctx.clone(), config.ma_length),
            long_roc: RocComponent::new(ctx.clone(), config.long_roc_length),
            short_roc: RocComponent::new(ctx.clone(), config.short_roc_length),
            config,
        };
    }
}

impl Component<(), Option<f64>> for CcIndicator {
    fn next(&mut self, _: ()) -> Option<f64> {
        let src = self.config.src.next(());

        let long_roc = self.long_roc.next(src);
        let short_roc = self.short_roc.next(src);
        let roc = match (long_roc, short_roc) {
            (Some(long_roc), Some(short_roc)) => Some(long_roc + short_roc),
            _ => None,
        };
        let curve = self.ma.next(roc);

        return curve;
    }
}
