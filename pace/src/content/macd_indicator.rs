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

pub struct MacdIndicatorConfig {
    pub short_src: AnySrcComponent,
    pub long_src: AnySrcComponent,
    pub short_ma: AnyMaComponent,
    pub long_ma: AnyMaComponent,
    pub signal_ma: AnyMaComponent,
}

impl ComponentDefault for MacdIndicatorConfig {
    fn default(ctx: ComponentContext) -> Self {
        Self {
            short_ma: MaComponent::build(ctx.clone(), MaKind::EMA, 12),
            long_ma: MaComponent::build(ctx.clone(), MaKind::EMA, 26),
            short_src: SrcComponent::build(ctx.clone(), SrcKind::Close),
            long_src: SrcComponent::build(ctx.clone(), SrcKind::Close),
            signal_ma: MaComponent::build(ctx.clone(), MaKind::EMA, 9),
        }
    }
}

/// Moving Average Convergence Divergence Indicator.
///
/// Ported from https://www.tradingview.com/chart/?symbol=BITSTAMP%3ABTCUSD&solution=43000502344
pub struct MacdIndicator {
    pub config: MacdIndicatorConfig,
    pub ctx: ComponentContext,
}

impl MacdIndicator {
    pub fn new(ctx: ComponentContext, config: MacdIndicatorConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            config,
        };
    }
}

impl Component<(), (Option<f64>, Option<f64>)> for MacdIndicator {
    fn next(&mut self, _: ()) -> (Option<f64>, Option<f64>) {
        let short_ma_src = self.config.short_src.next(());
        let long_ma_src = self.config.long_src.next(());

        let short_ma = self.config.short_ma.next(short_ma_src);
        let long_ma = self.config.long_ma.next(long_ma_src);

        let macd = match (short_ma, long_ma) {
            (Some(short_ma), Some(long_ma)) => Some(short_ma - long_ma),
            _ => None,
        };

        let signal = self.config.signal_ma.next(macd);

        return (macd, signal);
    }
}
