use crate::{
    components::{
        component::Component, component_context::ComponentContext,
        fixed_value_cache_component::FixedValueCacheComponent,
    },
    ta::sma_component::SmaComponent,
};

/// Deviation.
pub struct DevComponent {
    pub length: usize,
    pub ctx: ComponentContext,
    sma: SmaComponent,
    input_cache: FixedValueCacheComponent,
}

impl DevComponent {
    /// Biased by default.
    pub fn new(ctx: ComponentContext, length: usize) -> Self {
        assert!(length > 0, "DevComponent must have a length of at least 1");
        return Self {
            ctx: ctx.clone(),
            length,
            sma: SmaComponent::new(ctx.clone(), length),
            input_cache: FixedValueCacheComponent::new(ctx.clone(), length),
        };
    }
}

impl Component<Option<f64>, Option<f64>> for DevComponent {
    fn next(&mut self, value: Option<f64>) -> Option<f64> {
        if self.length == 1 {
            return Some(0.0);
        }

        self.input_cache.next(value);

        let mean = self.sma.next(value);

        if mean.is_none() || !self.input_cache.is_filled() {
            return None;
        }

        let mean = mean.unwrap();

        let values = self.input_cache.all();
        let sum = values
            .iter()
            .map(|v| (v.unwrap_or(mean) - mean).abs())
            .sum::<f64>();

        let dev = sum / self.length as f64;
        return Some(dev);
    }
}
