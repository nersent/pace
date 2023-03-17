use crate::{
    asset,
    components::{
        batch_validator_component::BatchValidatorComponent, component::Component,
        component_context::ComponentContext, window_cache_component::WindowCacheComponent,
    },
};

/// Symmetrically Weighted Moving Average with fixed length: 4. Weights: [1/6, 2/6, 2/6, 1/6].
///
/// Same as PineScript `ta.swma(src)`. Similar to `ta.swma(src, length)`, but `length` is fixed and set on initialization.
pub struct SwmaComponent {
    pub length: usize,
    pub ctx: ComponentContext,
    input_cache: WindowCacheComponent<Option<f64>>,
    batch_validator: BatchValidatorComponent,
}

static WEIGHTS: [f64; 4] = [1.0 / 6.0, 2.0 / 6.0, 2.0 / 6.0, 1.0 / 6.0];

impl SwmaComponent {
    pub fn new(ctx: ComponentContext) -> Self {
        let length = 4;
        return Self {
            ctx: ctx.clone(),
            length,
            input_cache: WindowCacheComponent::new(ctx.clone(), length),
            batch_validator: BatchValidatorComponent::new(ctx.clone(), length),
        };
    }
}

impl Component<Option<f64>, Option<f64>> for SwmaComponent {
    fn next(&mut self, value: Option<f64>) -> Option<f64> {
        self.input_cache.next(value);
        let is_valid = self.batch_validator.next(value);

        if !self.ctx.at_length(self.length) || !is_valid {
            return None;
        }

        let values = self.input_cache.all();

        let swma = values.iter().enumerate().fold(0.0, |acc, (i, value)| {
            let value = value.unwrap();
            let weight = WEIGHTS[i];
            let weighted_value = value * weight;
            acc + weighted_value
        });

        return Some(swma);
    }
}