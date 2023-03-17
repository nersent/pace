use crate::components::{
    component::Component, component_context::ComponentContext,
    fixed_value_cache_component::FixedValueCacheComponent,
};

/// Percent rank.
pub struct PrankComponent {
    pub length: usize,
    pub ctx: ComponentContext,
    input_cache: FixedValueCacheComponent,
}

impl PrankComponent {
    pub fn new(ctx: ComponentContext, length: usize) -> Self {
        assert!(length >= 1, "PrankComponent length must be >= 1");
        return Self {
            ctx: ctx.clone(),
            length,
            input_cache: FixedValueCacheComponent::new(ctx.clone(), length + 1),
        };
    }
}

impl Component<Option<f64>, Option<f64>> for PrankComponent {
    fn next(&mut self, value: Option<f64>) -> Option<f64> {
        self.input_cache.next(value);

        if value.is_none() || !self.ctx.at_length(self.length + 1) {
            return None;
        }

        let last_value = value.unwrap();

        let values = self.input_cache.all();
        let values = &values[0..values.len() - 1];

        let count = values
            .iter()
            .filter(|v| {
                if let Some(v) = v {
                    return v <= &last_value;
                }
                return false;
            })
            .count() as f64;

        let percent = count / self.length as f64 * 100.0;

        return Some(percent);
    }
}
