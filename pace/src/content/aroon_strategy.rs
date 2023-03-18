use crate::{
    components::{component::Component, component_context::ComponentContext},
    strategy::trade::TradeDirection,
    ta::{
        cross_component::CrossComponent, cross_over_component::CrossOverComponent,
        cross_under_component::CrossUnderComponent,
    },
};

use super::aroon_indicator::AroonIndicatorRes;

pub struct AroonStrategyData {
    pub up_trend_strength: f64,
    pub down_trend_strength: f64,
    pub cross_mode: bool,
}

/// Custom Aroon Strategy. May be incorrect.
pub struct AroonStrategy {
    pub ctx: ComponentContext,
    pub data: AroonStrategyData,
    cross: CrossComponent,
    up_trend_confirmation: bool,
    down_trend_confirmation: bool,
}

impl AroonStrategy {
    pub fn new(ctx: ComponentContext) -> Self {
        return AroonStrategy {
            ctx: ctx.clone(),
            cross: CrossComponent::new(ctx.clone()),
            up_trend_confirmation: false,
            down_trend_confirmation: false,
            data: AroonStrategyData {
                up_trend_strength: 0.0,
                down_trend_strength: 0.0,
                cross_mode: false,
            },
        };
    }
}

impl Component<&AroonIndicatorRes, Option<TradeDirection>> for AroonStrategy {
    fn next(&mut self, aroon: &AroonIndicatorRes) -> Option<TradeDirection> {
        self.data.up_trend_strength = match (aroon.up, aroon.down) {
            (Some(up), Some(down)) => {
                if up > 50.0 && down < 50.0 {
                    1.0 - (100.0 - up) / 50.0
                } else {
                    0.0
                }
            }
            _ => 0.0,
        };

        self.data.down_trend_strength = match (aroon.up, aroon.down) {
            (Some(up), Some(down)) => {
                if down > 50.0 && up < 50.0 {
                    1.0 - (100.0 - down) / 50.0
                } else {
                    0.0
                }
            }
            _ => 0.0,
        };

        let cross = self.cross.next(aroon.down, aroon.up);

        if cross.is_some() {
            self.data.cross_mode = true;
        }

        let mut up_trend_confirmation = false;
        let mut down_trend_confirmation = false;

        if self.data.cross_mode {
            if self.data.up_trend_strength >= 1.0 {
                up_trend_confirmation = true;
                self.data.cross_mode = false;
            } else if self.data.down_trend_strength >= 1.0 {
                down_trend_confirmation = true;
                self.data.cross_mode = false;
            }
        }

        let result = if up_trend_confirmation {
            Some(TradeDirection::Long)
        } else if down_trend_confirmation {
            Some(TradeDirection::Short)
        } else {
            None
        };

        return result;
    }
}
