use crate::{
    core::{context::Context, incremental::Incremental},
    pinescript::common::PineScriptFloat64,
};

/// Similar to PineScript `ta.tr(handle_na)`, but it requires for all values to be provided.
pub fn true_range(
    current_high: f64,
    current_low: f64,
    prev_high: f64,
    prev_low: f64,
    prev_close: f64,
    handle_na: bool,
) -> f64 {
    if prev_high.is_nan() || prev_low.is_nan() || prev_close.is_nan() {
        if handle_na {
            return current_high - current_low;
        } else {
            return f64::NAN;
        }
    }

    return f64::max(
        f64::max(
            current_high - current_low,
            f64::abs(current_high - prev_close),
        ),
        f64::abs(current_low - prev_close),
    );
}

/// True Range.
///
/// Similar to PineScript `ta.tr(handle_na)`, but `handle_na` is set on initialization.
pub struct Tr {
    pub ctx: Context,
    /// How NaN values are handled. if `true`, and previous day's close is NaN then tr would be calculated as current day high-low. Otherwise (if `false`) tr would return None in such cases.
    pub handle_na: bool,
}

impl Tr {
    // handle_na is false by default
    pub fn new(ctx: Context, handle_na: bool) -> Self {
        return Self {
            ctx: ctx.clone(),
            handle_na,
        };
    }
}

impl Incremental<(), f64> for Tr {
    fn next(&mut self, _: ()) -> f64 {
        let bar = &self.ctx.bar;

        return true_range(
            bar.high(),
            bar.low(),
            self.ctx.high(1),
            self.ctx.low(1),
            self.ctx.close(1),
            self.handle_na,
        );
    }
}
