use crate::{
    common::float_series::FloatSeries,
    core::{context::Context, incremental::Incremental},
};

use super::{common::var_from_mean, mean::Mean, welfords_var::WelfordsVar};

// Variance.
///
/// For O(1) complexity, use `fast`. By default it's `false`.
///
/// Compared to `ta::var`, this component calculates variance based on entire history of values.
///
/// Not the same as PineScript `ta.var`.
pub struct Var {
    pub ctx: Context,
    pub fast: bool,
    input_series: FloatSeries,
    welfords_var: WelfordsVar,
    mean: Mean,
}

impl Var {
    pub fn build(ctx: Context, fast: bool) -> Self {
        return Self {
            ctx: ctx.clone(),
            fast,
            welfords_var: WelfordsVar::new(ctx.clone()),
            input_series: FloatSeries::new(ctx.clone()),
            mean: Mean::new(ctx.clone()),
        };
    }

    pub fn new(ctx: Context) -> Self {
        return Self::build(ctx, false);
    }

    pub fn fast(ctx: Context) -> Self {
        return Self::build(ctx, true);
    }
}

impl Incremental<f64, f64> for Var {
    fn next(&mut self, value: f64) -> f64 {
        if self.fast {
            return self.welfords_var.next(value);
        }

        self.input_series.next(value);

        let mean = self.mean.next(value);
        let values = &self.input_series.values;

        return var_from_mean(values, mean);
    }
}
