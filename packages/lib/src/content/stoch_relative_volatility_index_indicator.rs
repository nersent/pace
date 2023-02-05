use crate::base::{
    asset::source::{Source, SourceKind},
    components::{component_context::ComponentContext, component_default::ComponentDefault},
    ta::{
        rsi_component::RelativeStrengthIndexComponent, sma_component::SimpleMovingAverageComponent,
        stoch_component::StochComponent,
    },
};

pub struct StochRelativeStrengthIndexIndicatorConfig {
    pub length_rsi: usize,
    pub length_stoch: usize,
    pub smooth_k: usize,
    pub smooth_d: usize,
    pub src: Source,
}

impl ComponentDefault for StochRelativeStrengthIndexIndicatorConfig {
    fn default(ctx: ComponentContext) -> Self {
        return StochRelativeStrengthIndexIndicatorConfig {
            length_rsi: 14,
            length_stoch: 14,
            smooth_k: 3,
            smooth_d: 3,
            src: Source::from_kind(ctx, SourceKind::Close),
        };
    }
}

pub struct StochRelativeStrengthIndexIndicator {
    config: StochRelativeStrengthIndexIndicatorConfig,
    ctx: ComponentContext,
    rsi: RelativeStrengthIndexComponent,
    k_stoch: StochComponent,
    k_sma: SimpleMovingAverageComponent,
    d_sma: SimpleMovingAverageComponent,
}

pub struct StochRelativeStrengthIndexIndicatorResult {
    pub k: Option<f64>,
    pub d: Option<f64>,
}

impl StochRelativeStrengthIndexIndicator {
    pub fn new(ctx: ComponentContext, config: StochRelativeStrengthIndexIndicatorConfig) -> Self {
        return StochRelativeStrengthIndexIndicator {
            ctx: ctx.clone(),
            rsi: RelativeStrengthIndexComponent::new(ctx.clone(), config.length_rsi),
            k_stoch: StochComponent::new(ctx.clone(), config.length_stoch),
            k_sma: SimpleMovingAverageComponent::new(ctx.clone(), config.smooth_k),
            d_sma: SimpleMovingAverageComponent::new(ctx.clone(), config.smooth_d),
            config,
        };
    }

    pub fn next(&mut self) -> StochRelativeStrengthIndexIndicatorResult {
        self.ctx.assert();
        let src = self.config.src.get();
        let rsi = self.rsi.next(src);

        let k_stoch = self.k_stoch.next(rsi, rsi, rsi);
        let k_sma = self.k_sma.next(k_stoch);
        let d_sma = self.d_sma.next(k_sma);

        return StochRelativeStrengthIndexIndicatorResult { k: k_sma, d: d_sma };
    }
}
