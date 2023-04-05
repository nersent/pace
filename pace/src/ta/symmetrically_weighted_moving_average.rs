use crate::{
    common::float_series::FloatSeries,
    core::{context::Context, incremental::Incremental},
    pinescript::common::PineScriptFloat64,
};

/// Symmetrically Weighted Moving Average with fixed length: 4. Weights: [1/6, 2/6, 2/6, 1/6].
///
/// Same as PineScript `ta.swma(src)`. Similar to `ta.swma(src, length)`, but `length` is fixed and set on initialization.
pub struct Swma {
    pub length: usize,
    pub ctx: Context,
    series: FloatSeries,
}

static WEIGHTS: [f64; 4] = [1.0 / 6.0, 2.0 / 6.0, 2.0 / 6.0, 1.0 / 6.0];

impl Swma {
    pub fn new(ctx: Context) -> Self {
        let length = 4;
        return Self {
            ctx: ctx.clone(),
            length,
            series: FloatSeries::new(ctx.clone()),
        };
    }
}

impl Incremental<f64, f64> for Swma {
    fn next(&mut self, value: f64) -> f64 {
        self.series.next(value);

        if !self.series.is_filled(self.length) {
            return f64::NAN;
        }

        let mut swma = 0.0;

        for (i, value) in self.series.window(self.length).iter().enumerate() {
            let weight = WEIGHTS[i];
            let weighted_value = value * weight;
            swma += weighted_value;
        }

        return swma;
    }
}
