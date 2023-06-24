use crate::{
    common::float_series::FloatSeries,
    core::{context::Context, incremental::Incremental},
};

pub struct Linreg {
    pub ctx: Context,
    pub length: usize,
    pub offset: i32,
    src_cache: FloatSeries,
}

impl Linreg {
    pub fn new(ctx: Context, length: usize, offset: i32) -> Self {
        Self {
            ctx: ctx.clone(),
            src_cache: FloatSeries::new(ctx.clone()),
            length,
            offset,
        }
    }
}

/*pinescript
ta.linreg

Linear regression curve. A line that best fits the prices specified over a user-defined time period. It is calculated using the least squares method. The result of this function is calculated using the formula: linreg = intercept + slope * (length - 1 - offset), where intercept and slope are the values calculated with the least squares method on `source` series.
ta.linreg(source, length, offset) → series float
RETURNS
Linear regression curve.
ARGUMENTS
source (series int/float) Source series.
length (series int) Number of bars (length).
offset (simple int) Offset.
*/
impl Incremental<f64, f64> for Linreg {
    fn next(&mut self, src: f64) -> f64 {
        self.src_cache.next(src);

        if !self.src_cache.is_filled(self.length) {
            return f64::NAN;
        }

        let window: &[f64] = self.src_cache.window(self.length);

        let sum_x = (0..self.length).map(|x| x as f64).sum::<f64>();
        let sum_y = window.iter().sum::<f64>();

        let sum_xy = window
            .iter()
            .enumerate()
            .map(|(i, x)| (i as f64) * x)
            .sum::<f64>();

        let sum_x2 = window
            .iter()
            .enumerate()
            .map(|(i, _)| (i as f64) * (i as f64))
            .sum::<f64>();

        let slope = (self.length as f64 * sum_xy - sum_x * sum_y)
            / (self.length as f64 * sum_x2 - sum_x * sum_x);

        let intercept = (sum_y - slope * sum_x) / self.length as f64;

        return intercept + slope * (self.length as f64 - 1.0 - self.offset as f64);
    }
}
