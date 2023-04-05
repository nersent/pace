use crate::{
    common::float_series::FloatSeries,
    core::{context::Context, incremental::Incremental},
    pinescript::common::PineScriptFloat64,
};

/// Weighted Moving Average. In wma weighting factors decrease in arithmetical progression.
///
/// Same as PineScript `ta.wma(src)`. Similar to `ta.wma(src, length)`, but `length` is fixed and set on initialization.
pub struct Wma {
    pub length: usize,
    pub ctx: Context,
    series: FloatSeries,
    weights: Vec<f64>,
    norm: f64,
    is_initialized: bool,
    non_nan_count: usize,
}

impl Wma {
    pub fn new(ctx: Context, length: usize) -> Self {
        assert!(length >= 1, "Wma must have a length of at least 1");
        let mut _self = Self {
            ctx: ctx.clone(),
            length,
            series: FloatSeries::new(ctx.clone()),
            weights: vec![0.0; length],
            norm: 0.0,
            is_initialized: false,
            non_nan_count: 0,
        };

        _self.init_weights();

        return _self;
    }

    fn init_weights(&mut self) {
        for i in 1..=self.length {
            let weight = i as f64 * self.length as f64;
            self.weights[i - 1] = weight;
            self.norm += weight;
        }
    }
}

impl Incremental<f64, f64> for Wma {
    fn next(&mut self, value: f64) -> f64 {
        self.series.next(value);

        if !self.is_initialized {
            if !value.is_nan() {
                self.non_nan_count += 1;
            }

            if self.series.is_filled(self.length) && self.non_nan_count >= self.length {
                self.is_initialized = true;
            } else {
                return f64::NAN;
            }
        }

        if value.is_nan() {
            return f64::NAN;
        }

        let mut sum = 0.0;

        for (i, item) in self.series.window(self.length).iter().enumerate() {
            sum += item.ps_nz() * self.weights[i];
        }

        return sum / self.norm;
    }
}
