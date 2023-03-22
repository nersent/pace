use crate::{
    common::window_cache::WindowCache,
    core::{context::Context, incremental::Incremental},
    testing::comparison::FloatComparison,
};

use super::simple_moving_average::Sma;

/// Standard Deviation.
///
/// Compared to `statistics::stdev`, this  calculates stdev based on a sliding window.
///
/// Same as PineScript `ta.stdev(src)`. Similar to `ta.stdev(src, length)`, but `length` is fixed and set on initialization.
pub struct Stdev {
    pub length: usize,
    pub ctx: Context,
    /// If `is_biased` is true, function will calculate using a biased estimate of the entire population, if false - unbiased estimate of a sample.
    pub is_biased: bool,
    sma: Sma,
    input_cache: WindowCache<Option<f64>>,
}

impl Stdev {
    // Biased by default.
    pub fn new(ctx: Context, length: usize, is_biased: bool) -> Self {
        assert!(length >= 1, "Stdev must have a length of at least 1");
        return Self {
            ctx: ctx.clone(),
            length,
            is_biased,
            sma: Sma::new(ctx.clone(), length),
            input_cache: WindowCache::new(ctx.clone(), length),
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

impl Incremental<Option<f64>, Option<f64>> for Stdev {
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
