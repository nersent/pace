use crate::components::{component_context::ComponentContext, component_default::ComponentDefault};

use super::common::omega_ratio;

pub struct OmegaRatioComponentConfig {
    pub risk_free_rate: f64,
}

impl Default for OmegaRatioComponentConfig {
    fn default() -> Self {
        return Self {
            risk_free_rate: 0.0,
        };
    }
}

pub struct OmegaRatioComponent {
    pub config: OmegaRatioComponentConfig,
    pub ctx: ComponentContext,
    pub positive_returns_sum: f64,
    pub negative_returns_sum: f64,
}

impl OmegaRatioComponent {
    pub fn new(ctx: ComponentContext, config: OmegaRatioComponentConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            config,
            positive_returns_sum: 0.0,
            negative_returns_sum: 0.0,
        };
    }

    pub fn next(&mut self, returns: f64) -> f64 {
        if returns > 0.0 {
            self.positive_returns_sum += returns;
        } else {
            self.negative_returns_sum += returns.abs();
        }

        return omega_ratio(self.positive_returns_sum, self.negative_returns_sum, 0.0);
    }
}
