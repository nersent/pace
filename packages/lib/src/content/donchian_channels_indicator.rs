use crate::base::{
    components::{component_context::ComponentContext, component_default::ComponentDefault},
    ta::bars::{compute_highest, compute_lowest},
};

pub struct DonchianChannelsIndicatorConfig {
    pub length: usize,
}

impl ComponentDefault for DonchianChannelsIndicatorConfig {
    fn default(ctx: ComponentContext) -> Self {
        Self { length: 20 }
    }
}

pub struct DonchianChannelsIndicatorResult {
    pub upper: Option<f64>,
    pub basis: Option<f64>,
    pub lower: Option<f64>,
}

pub struct DonchianChannelsIndicator {
    pub config: DonchianChannelsIndicatorConfig,
    ctx: ComponentContext,
}

impl DonchianChannelsIndicator {
    pub fn new(ctx: ComponentContext, config: DonchianChannelsIndicatorConfig) -> Self {
        return DonchianChannelsIndicator {
            ctx: ctx.clone(),
            config,
        };
    }

    pub fn next(&mut self) -> DonchianChannelsIndicatorResult {
        self.ctx.assert();
        let ctx = self.ctx.get();

        if (!ctx.at_length(self.config.length)) {
            return DonchianChannelsIndicatorResult {
                upper: None,
                basis: None,
                lower: None,
            };
        }

        let upper = compute_highest(ctx.prev_highs(self.config.length));
        let lower = compute_lowest(ctx.prev_lows(self.config.length));
        let basis = match (upper, lower) {
            (Some(upper), Some(lower)) => Some((upper + lower) / 2.0),
            _ => None,
        };

        return DonchianChannelsIndicatorResult {
            upper,
            basis,
            lower,
        };
    }
}
