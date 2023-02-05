use crate::base::{
    asset::source::{Source, SourceKind},
    components::{component_context::ComponentContext, component_default::ComponentDefault},
    ta::{
        ema_component::ExponentialMovingAverageComponent,
        stdev_component::StandardDeviationComponent,
    },
};

pub struct RelativeVolatilityIndexIndicatorConfig {
    pub length: usize,
    pub ma_length: usize,
    pub src: Source,
}

impl ComponentDefault for RelativeVolatilityIndexIndicatorConfig {
    fn default(ctx: ComponentContext) -> Self {
        Self {
            length: 10,
            ma_length: 14,
            src: Source::from_kind(ctx.clone(), SourceKind::Close),
        }
    }
}

pub struct RelativeVolatilityIndexIndicator {
    pub config: RelativeVolatilityIndexIndicatorConfig,
    ctx: ComponentContext,
    stdev: StandardDeviationComponent,
    upper_ema: ExponentialMovingAverageComponent,
    lower_ema: ExponentialMovingAverageComponent,
    prev_src: Option<f64>,
}

impl RelativeVolatilityIndexIndicator {
    pub fn new(ctx: ComponentContext, config: RelativeVolatilityIndexIndicatorConfig) -> Self {
        return RelativeVolatilityIndexIndicator {
            ctx: ctx.clone(),
            stdev: StandardDeviationComponent::new(ctx.clone(), config.length, true),
            upper_ema: ExponentialMovingAverageComponent::new(ctx.clone(), config.ma_length),
            lower_ema: ExponentialMovingAverageComponent::new(ctx.clone(), config.ma_length),
            config,
            prev_src: None,
        };
    }

    pub fn next(&mut self) -> Option<f64> {
        self.ctx.assert();
        let ctx = self.ctx.get();

        let src = self.config.src.get();
        let stdev = self.stdev.next(src);
        let src_change = match (src, self.prev_src) {
            (Some(src), Some(prev_src)) => Some(src - prev_src),
            _ => None,
        };

        let (upper, lower) = match (src_change) {
            (Some(change)) => {
                let upper = if change <= 0.0 { Some(0.0) } else { stdev };
                let lower = if change > 0.0 { Some(0.0) } else { stdev };
                (upper, lower)
            }
            _ => (None, None),
        };

        let upper = self.upper_ema.next(upper);
        let lower = self.lower_ema.next(lower);

        let rvi = match (upper, lower) {
            (Some(upper), Some(lower)) => {
                if upper == -lower {
                    None
                } else {
                    Some(upper / (upper + lower) * 100.0)
                }
            }
            _ => None,
        };

        self.prev_src = src;

        return rvi;
    }
}
