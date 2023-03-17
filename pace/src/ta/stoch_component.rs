use crate::components::{
    component::Component, component_context::ComponentContext,
    fixed_value_cache_component::FixedValueCacheComponent,
};

use super::stoch::stoch;

pub struct StochComponent {
    pub length: usize,
    pub ctx: ComponentContext,
    prev_stoch: Option<f64>,
    high_input_cache: FixedValueCacheComponent,
    low_input_cache: FixedValueCacheComponent,
}

impl StochComponent {
    pub fn new(ctx: ComponentContext, length: usize) -> Self {
        assert!(length >= 1, "StochComponent length must be >= 1");
        return Self {
            ctx: ctx.clone(),
            length,
            prev_stoch: None,
            high_input_cache: FixedValueCacheComponent::new(ctx.clone(), length),
            low_input_cache: FixedValueCacheComponent::new(ctx.clone(), length),
        };
    }
}

impl Component<(Option<f64>, Option<f64>, Option<f64>), Option<f64>> for StochComponent {
    /// value, high, low
    fn next(&mut self, (value, high, low): (Option<f64>, Option<f64>, Option<f64>)) -> Option<f64> {
        self.high_input_cache.next(high);
        self.low_input_cache.next(low);

        if !self.ctx.at_length(self.length) {
            return None;
        }

        let _stoch = stoch(
            value,
            self.high_input_cache.all(),
            self.low_input_cache.all(),
            self.prev_stoch,
        );
        self.prev_stoch = _stoch;

        return _stoch;
    }
}
