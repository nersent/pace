use crate::base::{
    asset::source::{Source, SourceKind},
    components::{component_context::ComponentContext, component_default::ComponentDefault},
    ta::{ma::MovingAverageKind, ma_component::MovingAverageComponent},
};

pub struct AwesomeOscillatorIndicatorConfig {
    pub short_length: usize,
    pub long_length: usize,
    pub short_source: Source,
    pub long_source: Source,
    pub short_ma_type: MovingAverageKind,
    pub long_ma_type: MovingAverageKind,
}

impl ComponentDefault for AwesomeOscillatorIndicatorConfig {
    fn default(ctx: ComponentContext) -> Self {
        Self {
            short_length: 5,
            long_length: 34,
            short_source: Source::from_kind(ctx.clone(), SourceKind::HL2),
            long_source: Source::from_kind(ctx.clone(), SourceKind::HL2),
            short_ma_type: MovingAverageKind::SMA,
            long_ma_type: MovingAverageKind::SMA,
        }
    }
}

pub struct AwesomeOscillatorIndicator {
    config: AwesomeOscillatorIndicatorConfig,
    ctx: ComponentContext,
    short_ma: MovingAverageComponent,
    long_ma: MovingAverageComponent,
    prev_ao: Option<f64>,
}

impl AwesomeOscillatorIndicator {
    pub fn new(ctx: ComponentContext, config: AwesomeOscillatorIndicatorConfig) -> Self {
        return AwesomeOscillatorIndicator {
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

        let short_ma_src = self.config.short_source.get();
        let long_ma_src = self.config.long_source.get();

        let short_ma = self.short_ma.next(short_ma_src);
        let long_ma = self.long_ma.next(long_ma_src);

        let ao = match (short_ma, long_ma) {
            (Some(short_ma), Some(long_ma)) => Some(short_ma - long_ma),
            _ => None,
        };

        let osc = match (ao, self.prev_ao) {
            (Some(ao), Some(prev_ao)) => Some(ao - prev_ao),
            _ => None,
        };

        self.prev_ao = ao;

        return osc;
    }
}
