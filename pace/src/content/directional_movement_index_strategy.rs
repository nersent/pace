use crate::{
    components::{component::Component, component_context::ComponentContext},
    strategy::trade::TradeDirection,
    ta::{cross::CrossMode, cross_component::CrossComponent},
};

use super::directional_movement_index_indicator::DmiIndicatorData;

pub static DMI_THRESHOLD_STRONG_TREND: f64 = 25.0;
pub static DMI_THRESHOLD_WEAK_TREND: f64 = 20.0;

pub struct DmiStrategyConfig {
    pub threshold_strong_trend: f64,
    pub threshold_weak_trend: f64,
}

impl Default for DmiStrategyConfig {
    fn default() -> Self {
        return Self {
            threshold_strong_trend: DMI_THRESHOLD_STRONG_TREND,
            threshold_weak_trend: DMI_THRESHOLD_WEAK_TREND,
        };
    }
}

/// Custom Directional Movement Index Strategy. May be incorrect.
pub struct DmiStrategy {
    pub config: DmiStrategyConfig,
    pub ctx: ComponentContext,
    cross: CrossComponent,
}

impl DmiStrategy {
    pub fn new(ctx: ComponentContext, config: DmiStrategyConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            cross: CrossComponent::new(ctx.clone()),
            config,
        };
    }
}

impl Component<&DmiIndicatorData, Option<TradeDirection>> for DmiStrategy {
    fn next(&mut self, dmi: &DmiIndicatorData) -> Option<TradeDirection> {
        let is_strong_trend = dmi
            .adx
            .map(|x| x > self.config.threshold_strong_trend)
            .unwrap_or(false);

        let is_weak_trend = dmi
            .adx
            .map(|x| x < self.config.threshold_weak_trend)
            .unwrap_or(false);

        let plus_minus_cross = self.cross.next(dmi.plus, dmi.minus);

        let mut result: Option<TradeDirection> = None;

        if is_strong_trend {
            if let Some(plus_minus_cross) = plus_minus_cross {
                result = match plus_minus_cross {
                    CrossMode::Over => Some(TradeDirection::Long),
                    CrossMode::Under => Some(TradeDirection::Short),
                }
            }
        }

        return result;
    }
}
