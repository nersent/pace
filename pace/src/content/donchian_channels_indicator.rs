use crate::{
    components::{component::Component, component_context::ComponentContext},
    pinescript::math::ps_add,
    ta::{highest_component::HighestComponent, lowest_component::LowestComponent},
};

use super::directional_movement_index_indicator::DmiIndicatorData;

pub struct DcIndicatorConfig {
    pub length: usize,
}

impl Default for DcIndicatorConfig {
    fn default() -> Self {
        Self { length: 20 }
    }
}

pub struct DcIndicatorData {
    pub upper: Option<f64>,
    pub basis: Option<f64>,
    pub lower: Option<f64>,
}

pub struct DcIndicator {
    pub config: DcIndicatorConfig,
    pub ctx: ComponentContext,
    highest: HighestComponent,
    lowest: LowestComponent,
}

impl DcIndicator {
    pub fn new(ctx: ComponentContext, config: DcIndicatorConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            highest: HighestComponent::new(ctx.clone(), config.length),
            lowest: LowestComponent::new(ctx.clone(), config.length),
            config,
        };
    }
}

impl Component<(), DcIndicatorData> for DcIndicator {
    fn next(&mut self, _: ()) -> DcIndicatorData {
        let upper = self.highest.next(self.ctx.high());
        let lower = self.lowest.next(self.ctx.low());

        let basis = ps_add(upper, lower).map(|x| x / 2.0);

        return DcIndicatorData {
            upper,
            basis,
            lower,
        };
    }
}
