use crate::{
    components::{
        component::Component, component_context::ComponentContext,
        fixed_value_cache_component::FixedValueCacheComponent,
    },
    ta::sma_component::SmaComponent,
    utils::comparison::FloatComparison,
};

pub struct StdevComponent {
    pub length: usize,
    pub is_biased: bool,
    pub ctx: ComponentContext,
    sma: SmaComponent,
    input_cache: FixedValueCacheComponent,
}

impl StdevComponent {
    // Biased by default.
    pub fn new(ctx: ComponentContext, length: usize, is_biased: bool) -> Self {
        assert!(
            length > 0,
            "StdevComponent must have a length of at least 1"
        );
        return Self {
            ctx: ctx.clone(),
            length,
            is_biased,
            sma: SmaComponent::new(ctx.clone(), length),
            input_cache: FixedValueCacheComponent::new(ctx.clone(), length),
        };
    }

    fn compute_sum(fst: f64, snd: f64) -> f64 {
        let sum = fst + snd;
        if sum.compare_with_precision(0.0, 1e-10) {
            return 0.0;
        }
        return sum;
    }
}

impl Component<Option<f64>, Option<f64>> for StdevComponent {
    fn next(&mut self, value: Option<f64>) -> Option<f64> {
        if self.length == 1 {
            if self.is_biased {
                return Some(0.0);
            } else {
                return None;
            }
        }

        self.input_cache.next(value);

        let mean = self.sma.next(value);

        mean?;

        let mean = -mean.unwrap();

        let values = self.input_cache.all();
        let sum = values
            .iter()
            .map(|v| {
                if let Some(v) = v {
                    let sum = Self::compute_sum(*v, mean);
                    sum.powf(2.0)
                } else {
                    0.0
                }
            })
            .sum::<f64>();

        let stdev = if self.is_biased {
            (sum / self.length as f64).sqrt()
        } else {
            (sum / (self.length - 1) as f64).sqrt()
        };

        return Some(stdev);
    }
}
