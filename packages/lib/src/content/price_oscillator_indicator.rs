use crate::base::{
    asset::source::{Source, SourceKind},
    components::{component_context::ComponentContext, component_default::ComponentDefault},
    ta::{ma::MovingAverageKind, ma_component::MovingAverageComponent},
};

pub struct PriceOscillatorIndicatorConfig {
    pub short_length: usize,
    pub long_length: usize,
    pub src: Source,
    pub short_ma_type: MovingAverageKind,
    pub long_ma_type: MovingAverageKind,
}

impl ComponentDefault for PriceOscillatorIndicatorConfig {
    fn default(ctx: ComponentContext) -> Self {
        Self {
            short_length: 10,
            long_length: 21,
            src: Source::from_kind(ctx.clone(), SourceKind::Close),
            short_ma_type: MovingAverageKind::SMA,
            long_ma_type: MovingAverageKind::SMA,
        }
    }
}

pub struct PriceOscillatorIndicator {
    config: PriceOscillatorIndicatorConfig,
    ctx: ComponentContext,
    short_ma: MovingAverageComponent,
    long_ma: MovingAverageComponent,
    prev_ao: Option<f64>,
}

impl PriceOscillatorIndicator {
    pub fn new(ctx: ComponentContext, config: PriceOscillatorIndicatorConfig) -> Self {
        return PriceOscillatorIndicator {
            ctx: ctx.clone(),
            short_ma: MovingAverageComponent::new(
                ctx.clone(),
                config.short_length,
                config.short_ma_type,
            ),
            long_ma: MovingAverageComponent::new(
                ctx.clone(),
                config.long_length,
                config.long_ma_type,
            ),
            config,
            prev_ao: None,
        };
    }

    pub fn next(&mut self) -> Option<f64> {
        self.ctx.assert();

        let ctx = self.ctx.get();

        let src = self.config.src.get();

        let short_ma = self.short_ma.next(src);
        let long_ma = self.long_ma.next(src);

        let po: Option<f64> = match (short_ma, long_ma) {
            (Some(short_ma), Some(long_ma)) => Some((short_ma - long_ma) / long_ma * 100.0),
            _ => None,
        };

        return po;
    }
}
