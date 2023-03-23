use crate::{
    common::{window_cache::WindowCache, window_validator::WindowValidator},
    core::{context::Context, incremental::Incremental},
};

/// Weighted Moving Average. In wma weighting factors decrease in arithmetical progression.
///
/// Same as PineScript `ta.wma(src)`. Similar to `ta.wma(src, length)`, but `length` is fixed and set on initialization.
pub struct Wma {
    pub length: usize,
    pub ctx: Context,
    input_cache: WindowCache<Option<f64>>,
    batch_validator: WindowValidator,
}

impl Wma {
    pub fn new(ctx: Context, length: usize) -> Self {
        assert!(length >= 1, "Wma must have a length of at least 1");
        return Self {
            ctx: ctx.clone(),
            length,
            input_cache: WindowCache::new(ctx.clone(), length),
            batch_validator: WindowValidator::new(ctx.clone(), length),
        };
    }
}

impl Incremental<Option<f64>, Option<f64>> for Wma {
    fn next(&mut self, value: Option<f64>) -> Option<f64> {
        self.input_cache.next(value);
        let is_valid = self.batch_validator.next(value);

        if !self.ctx.bar.at_length(self.length) || !is_valid {
            return None;
        }

        let values = self.input_cache.all();

        let (sum, norm) = values
            .iter()
            .rev()
            .enumerate()
            .fold((0.0, 0.0), |acc, (i, value)| {
                let value = value.unwrap();
                let weight = ((self.length - i) * self.length) as f64;
                let weighted_value = value * weight;
                (acc.0 + weighted_value, acc.1 + weight)
            });

        let wma = sum / norm;

        return Some(wma);
    }
}
