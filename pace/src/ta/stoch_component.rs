use crate::components::{
    component::Component, component_context::ComponentContext,
    window_cache_component::WindowCacheComponent,
};

use super::stoch::stoch;

/// Stochastic.
///
/// Same as PineScript `ta.stoch(src, high, low)`. Similar to `ta.stoch(src, high, low, length)`, but `length` is fixed and set on initialization.
pub struct StochComponent {
    pub length: usize,
    pub ctx: ComponentContext,
    prev_stoch: Option<f64>,
    high_input_cache: WindowCacheComponent<Option<f64>>,
    low_input_cache: WindowCacheComponent<Option<f64>>,
}

impl StochComponent {
    pub fn new(ctx: ComponentContext, length: usize) -> Self {
        assert!(
            length >= 1,
            "StochComponent must have a length of at least 1"
        );
        return Self {
            ctx: ctx.clone(),
            length,
            prev_stoch: None,
            high_input_cache: WindowCacheComponent::new(ctx.clone(), length),
            low_input_cache: WindowCacheComponent::new(ctx.clone(), length),
        };
    }
}

impl Component<(Option<f64>, Option<f64>, Option<f64>), Option<f64>> for StochComponent {
    /// Input: `src, high, low`.
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
