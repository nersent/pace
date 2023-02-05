use crate::base::{
    components::{component_context::ComponentContext, component_default::ComponentDefault},
    ta::{
        ema_component::ExponentialMovingAverageComponent, ma::MovingAverageKind,
        ma_component::MovingAverageComponent,
    },
};

pub struct VolumeOscillatorIndicatorConfig {
    pub short_length: usize,
    pub long_length: usize,
    pub short_ma_kind: MovingAverageKind,
    pub long_ma_kind: MovingAverageKind,
}

impl ComponentDefault for VolumeOscillatorIndicatorConfig {
    fn default(ctx: ComponentContext) -> Self {
        Self {
            short_length: 5,
            long_length: 10,
            short_ma_kind: MovingAverageKind::EMA,
            long_ma_kind: MovingAverageKind::EMA,
        }
    }
}

pub struct VolumeOscillatorIndicator {
    config: VolumeOscillatorIndicatorConfig,
    ctx: ComponentContext,
    short_ma: MovingAverageComponent,
    long_ma: MovingAverageComponent,
}

impl VolumeOscillatorIndicator {
    pub fn new(ctx: ComponentContext, config: VolumeOscillatorIndicatorConfig) -> Self {
        return VolumeOscillatorIndicator {
            ctx: ctx.clone(),
            short_ma: MovingAverageComponent::new(
                ctx.clone(),
                config.short_length,
                config.short_ma_kind,
            ),
            long_ma: MovingAverageComponent::new(
                ctx.clone(),
                config.long_length,
                config.long_ma_kind,
            ),
            config,
        };
    }

    pub fn next(&mut self) -> Option<f64> {
        self.ctx.assert();

        let ctx = self.ctx.get();
        let volume = ctx.volume();

        let short_ma = self.short_ma.next(volume);
        let long_ma = self.long_ma.next(volume);

        let osc = match (short_ma, long_ma) {
            (Some(short_ma), Some(long_ma)) => {
                if long_ma == 0.0 {
                    None
                } else {
                    Some((short_ma - long_ma) / long_ma * 100.0)
                }
            }
            _ => None,
        };

        return osc;
    }
}
