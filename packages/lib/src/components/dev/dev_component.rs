use crate::{
    components::{
        batch_validator::recursive_batch_validator::RecursiveBatchValidator,
        component_context::ComponentContext, lifo::recursive_lifo::RecursiveLIFO,
        value_cache::fixed_value_cache_component::FixedValueCacheComponent,
    },
    math::comparison::FloatComparison,
    ta::moving_average::{
        rma_component::RunningMovingAverageComponent, sma_component::SimpleMovingAverageComponent,
    },
};

pub struct DeviationComponent {
    pub length: usize,
    ctx: ComponentContext,
    sma: SimpleMovingAverageComponent,
    input_cache: FixedValueCacheComponent,
}

impl DeviationComponent {
    // biased by default
    pub fn new(ctx: ComponentContext, length: usize) -> Self {
        assert!(
            length > 0,
            "DeviationComponent must have a length of at least 1"
        );
        return DeviationComponent {
            ctx: ctx.clone(),
            length,
            sma: SimpleMovingAverageComponent::new(ctx.clone(), length),
            input_cache: FixedValueCacheComponent::new(ctx.clone(), length),
        };
    }

    pub fn next(&mut self, value: Option<f64>) -> Option<f64> {
        self.ctx.assert();

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
