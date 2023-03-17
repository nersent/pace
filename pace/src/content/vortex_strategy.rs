use crate::{
    components::{component::Component, component_context::ComponentContext},
    strategy::trade::TradeDirection,
    ta::{cross::CrossMode, cross_component::CrossComponent},
};

use super::vortex_indicator::VortexIndicatorData;

pub struct VortexStrategy {
    pub ctx: ComponentContext,
    cross: CrossComponent,
}

impl VortexStrategy {
    pub fn new(ctx: ComponentContext) -> Self {
        return Self {
            ctx: ctx.clone(),
            cross: CrossComponent::new(ctx.clone()),
        };
    }
}

impl Component<&VortexIndicatorData, Option<TradeDirection>> for VortexStrategy {
    fn next(&mut self, vi: &VortexIndicatorData) -> Option<TradeDirection> {
        let vip_vim_cross = self.cross.next(vi.plus, vi.minus);

        let mut result: Option<TradeDirection> = None;

        if let Some(plus_minus_cross) = vip_vim_cross {
            result = match plus_minus_cross {
                CrossMode::Over => Some(TradeDirection::Long),
                CrossMode::Under => Some(TradeDirection::Short),
            }
        }

        return result;
    }
}
