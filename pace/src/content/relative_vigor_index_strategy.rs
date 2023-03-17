use crate::{
    components::{component::Component, component_context::ComponentContext},
    strategy::trade::TradeDirection,
    ta::{cross::CrossMode, cross_component::CrossComponent},
};

use super::relative_vigor_index_indicator::RvgiIndicatorData;

/// Custom Relative Vigor Index Strategy. May be incorrect.
pub struct RvgiStrategy {
    pub ctx: ComponentContext,
    cross: CrossComponent,
}

impl RvgiStrategy {
    pub fn new(ctx: ComponentContext) -> Self {
        return Self {
            ctx: ctx.clone(),
            cross: CrossComponent::new(ctx.clone()),
        };
    }
}

impl Component<&RvgiIndicatorData, Option<TradeDirection>> for RvgiStrategy {
    fn next(&mut self, rvgi: &RvgiIndicatorData) -> Option<TradeDirection> {
        let rvi_s_cross = self.cross.next(rvgi.rvi, rvgi.sig);

        let mut result: Option<TradeDirection> = None;

        if let Some(plus_minus_cross) = rvi_s_cross {
            result = match plus_minus_cross {
                CrossMode::Over => Some(TradeDirection::Long),
                CrossMode::Under => Some(TradeDirection::Short),
            }
        }

        return result;
    }
}
