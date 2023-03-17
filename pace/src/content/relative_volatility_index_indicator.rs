use crate::{
    components::{
        component::Component,
        component_context::ComponentContext,
        component_default::ComponentDefault,
        src::SrcKind,
        src_component::{AnySrcComponent, SrcComponent},
    },
    ta::{ema_component::EmaComponent, stdev_component::StdevComponent},
};

pub static RVI_MIN_VALUE: f64 = 0.0;
pub static RVI_MAX_VALUE: f64 = 100.0;

pub struct RviIndicatorConfig {
    pub length: usize,
    pub ma_length: usize,
    pub src: AnySrcComponent,
}

impl ComponentDefault for RviIndicatorConfig {
    fn default(ctx: ComponentContext) -> Self {
        Self {
            length: 10,
            ma_length: 14,
            src: SrcComponent::build(ctx.clone(), SrcKind::Close),
        }
    }
}

/// Relative Volatility Index Indicator.
///
/// Ported from https://www.tradingview.com/chart/?solution=43000594684
pub struct RviIndicator {
    pub config: RviIndicatorConfig,
    pub ctx: ComponentContext,
    stdev: StdevComponent,
    upper_ema: EmaComponent,
    lower_ema: EmaComponent,
    prev_src: Option<f64>,
}

impl RviIndicator {
    pub fn new(ctx: ComponentContext, config: RviIndicatorConfig) -> Self {
        return RviIndicator {
            ctx: ctx.clone(),
            stdev: StdevComponent::new(ctx.clone(), config.length, true),
            upper_ema: EmaComponent::new(ctx.clone(), config.ma_length),
            lower_ema: EmaComponent::new(ctx.clone(), config.ma_length),
            config,
            prev_src: None,
        };
    }
}

impl Component<(), Option<f64>> for RviIndicator {
    fn next(&mut self, _: ()) -> Option<f64> {
        let src = self.config.src.next(());
        let stdev = self.stdev.next(src);
        let src_change = match (src, self.prev_src) {
            (Some(src), Some(prev_src)) => Some(src - prev_src),
            _ => None,
        };

        let (upper, lower) = match src_change {
            Some(change) => {
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
