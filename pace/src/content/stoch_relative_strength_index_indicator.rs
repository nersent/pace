use crate::{
    components::{
        component::Component,
        component_context::ComponentContext,
        component_default::ComponentDefault,
        src::SrcKind,
        src_component::{AnySrcComponent, SrcComponent},
    },
    ta::{
        rsi_component::RsiComponent, sma_component::SmaComponent, stoch_component::StochComponent,
    },
};

pub static SRSI_MIN_VALUE: f64 = 0.0;
pub static SRSI_MAX_VALUE: f64 = 100.0;

pub struct SrsiIndicatorConfig {
    pub length_rsi: usize,
    pub length_stoch: usize,
    pub smooth_k: usize,
    pub smooth_d: usize,
    pub src: AnySrcComponent,
}

impl ComponentDefault for SrsiIndicatorConfig {
    fn default(ctx: ComponentContext) -> Self {
        return Self {
            length_rsi: 14,
            length_stoch: 14,
            smooth_k: 3,
            smooth_d: 3,
            src: SrcComponent::build(ctx.clone(), SrcKind::Close),
        };
    }
}

pub struct SrsiIndicatorData {
    pub k: Option<f64>,
    pub d: Option<f64>,
}

/// Stochastic Relative Strength Index Indicator.
///
/// Ported from https://www.tradingview.com/chart/?solution=43000502333
pub struct SrsiIndicator {
    pub config: SrsiIndicatorConfig,
    pub ctx: ComponentContext,
    rsi: RsiComponent,
    k_stoch: StochComponent,
    k_sma: SmaComponent,
    d_sma: SmaComponent,
}

impl SrsiIndicator {
    pub fn new(ctx: ComponentContext, config: SrsiIndicatorConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            rsi: RsiComponent::new(ctx.clone(), config.length_rsi),
            k_stoch: StochComponent::new(ctx.clone(), config.length_stoch),
            k_sma: SmaComponent::new(ctx.clone(), config.smooth_k),
            d_sma: SmaComponent::new(ctx.clone(), config.smooth_d),
            config,
        };
    }
}

impl Component<(), SrsiIndicatorData> for SrsiIndicator {
    fn next(&mut self, _: ()) -> SrsiIndicatorData {
        let src = self.config.src.next(());
        let rsi = self.rsi.next(src);

        let k_stoch = self.k_stoch.next((rsi, rsi, rsi));
        let k_sma = self.k_sma.next(k_stoch);
        let d_sma = self.d_sma.next(k_sma);

        return SrsiIndicatorData { k: k_sma, d: d_sma };
    }
}
