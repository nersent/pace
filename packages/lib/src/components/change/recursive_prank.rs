use crate::components::{
    batch_validator::recursive_batch_validator::RecursiveBatchValidator,
    component_context::ComponentContext, lifo::recursive_lifo::RecursiveLIFO,
    value_cache::fixed_value_cache_component::FixedValueCacheComponent,
};

pub struct RecursivePercentRank {
    length: usize,
    ctx: ComponentContext,
    count: f64,
    input_cache: FixedValueCacheComponent,
}

impl RecursivePercentRank {
    pub fn new(ctx: ComponentContext, length: usize) -> Self {
        assert!(length >= 1, "RecursivePercentRank length must be >= 1");
        return RecursivePercentRank {
            ctx: ctx.clone(),
            length,
            count: 0.0,
            input_cache: FixedValueCacheComponent::new(ctx.clone(), length + 1),
        };
    }

    pub fn next(&mut self, value: Option<f64>) -> Option<f64> {
        self.ctx.assert();

        self.input_cache.next(value);

        if value.is_none() || !self.ctx.get().at_length(self.length + 1) {
            return None;
        }

        let last_value = value.unwrap();
        let mut count: f64 = 0.0;

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
