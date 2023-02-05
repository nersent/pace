use crate::base::{
    asset::source::{Source, SourceKind},
    components::{component_context::ComponentContext, component_default::ComponentDefault},
    ta::{roc_component::RateOfChangeComponent, wma_component::WeightedMovingAverageComponent},
};

pub struct CoppockCurveIndicatorConfig {
    pub src: Source,
    pub long_roc_length: usize,
    pub short_roc_length: usize,
    pub ma_length: usize,
}

impl ComponentDefault for CoppockCurveIndicatorConfig {
    fn default(ctx: ComponentContext) -> Self {
        Self {
            ma_length: 10,
            long_roc_length: 14,
            short_roc_length: 11,
            src: Source::from_kind(ctx.clone(), SourceKind::Close),
        }
    }
}

pub struct CoppockCurveIndicator {
    pub config: CoppockCurveIndicatorConfig,
    ctx: ComponentContext,
    ma: WeightedMovingAverageComponent,
    long_roc: RateOfChangeComponent,
    short_roc: RateOfChangeComponent,
}

impl CoppockCurveIndicator {
    pub fn new(ctx: ComponentContext, config: CoppockCurveIndicatorConfig) -> Self {
        return CoppockCurveIndicator {
            ctx: ctx.clone(),
            ma: WeightedMovingAverageComponent::new(ctx.clone(), config.ma_length),
            long_roc: RateOfChangeComponent::new(ctx.clone(), config.long_roc_length),
            short_roc: RateOfChangeComponent::new(ctx.clone(), config.short_roc_length),
            config,
        };
    }

    pub fn next(&mut self) -> Option<f64> {
        self.ctx.assert();
        let ctx = self.ctx.get();

        let src = self.config.src.get();

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
