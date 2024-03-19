use crate::{
    core::{context::Context, incremental::Incremental},
    pinescript::common::PineScriptFloat64,
};

use super::simple_moving_average::Sma;

/*pine
// @function                An alternate EMA function to the `ta.ema()` built-in, which allows a `series float`
//                          length argument.
// @param src               (series int/float) Series of values to process.
// @param length            (series int/float) Number of bars (length).
// @returns                 (float) The exponentially weighted moving average of the `src`.
export ema2(series float src, series float length) =>
    float result = na
    float alpha  = 2.0 / (math.max(1.0, length) + 1.0)
    float beta   = 1.0 - alpha
    result      := alpha * src + beta * nz(result[1], src)
    */
pub struct Ema2 {
    pub ctx: Context,
    result: f64,
}

impl Ema2 {
    pub fn new(ctx: Context) -> Self {
        return Self {
            ctx: ctx.clone(),
            result: f64::NAN,
        };
    }
}

impl Incremental<(f64, usize), f64> for Ema2 {
    fn next(&mut self, (src, length): (f64, usize)) -> f64 {
        let prev_result = self.result;

        let alpha = 2.0 / (f64::max(1.0, length as f64) + 1.0);
        let beta = 1.0 - alpha;

        let result = alpha * src + beta * prev_result.ps_nz_with(src);

        self.result = result;

        return result;
    }
}
