use crate::{
    common::{window_cache::WindowCache, window_validator::WindowValidator},
    core::{context::Context, incremental::Incremental},
};

/// Symmetrically Weighted Moving Average with fixed length: 4. Weights: [1/6, 2/6, 2/6, 1/6].
///
/// Same as PineScript `ta.swma(src)`. Similar to `ta.swma(src, length)`, but `length` is fixed and set on initialization.
pub struct Swma {
    pub length: usize,
    pub ctx: Context,
    input_cache: WindowCache<Option<f64>>,
    batch_validator: WindowValidator,
}

static WEIGHTS: [f64; 4] = [1.0 / 6.0, 2.0 / 6.0, 2.0 / 6.0, 1.0 / 6.0];

impl Swma {
    pub fn new(ctx: Context) -> Self {
        let length = 4;
        return Self {
            ctx: ctx.clone(),
            length,
            input_cache: WindowCache::new(ctx.clone(), length),
            batch_validator: WindowValidator::new(ctx.clone(), length),
        };
    }
}

impl Incremental<Option<f64>, Option<f64>> for Swma {
    fn next(&mut self, value: Option<f64>) -> Option<f64> {
        self.input_cache.next(value);
        let is_valid = self.batch_validator.next(value);

        if !self.ctx.bar.at_length(self.length) || !is_valid {
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
