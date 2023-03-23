use crate::core::{context::Context, incremental::Incremental};

/// Similar to PineScript `ta.tr(handle_na)`, but it requires for all values to be provided.
pub fn true_range(
    current_high: f64,
    current_low: f64,
    prev_high: Option<f64>,
    prev_low: Option<f64>,
    prev_close: Option<f64>,
    handle_na: bool,
) -> Option<f64> {
    if prev_high.is_none() || prev_low.is_none() || prev_close.is_none() {
        if handle_na {
            return Some(current_high - current_low);
        } else {
            return None;
        }
    }

    let prev_close = prev_close.unwrap();

    return Some(f64::max(
        f64::max(
            current_high - current_low,
            f64::abs(current_high - prev_close),
        ),
        f64::abs(current_low - prev_close),
    ));
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
    pub fn new(ctx: Context, handle_na: bool) -> Self {
        return Self {
            ctx: ctx.clone(),
            handle_na,
        };
    }
}

impl Incremental<(), Option<f64>> for Tr {
    fn next(&mut self, _: ()) -> Option<f64> {
        return true_range(
            self.ctx.bar.high().unwrap(),
            self.ctx.bar.low().unwrap(),
            self.ctx.high(1),
            self.ctx.low(1),
            self.ctx.close(1),
            self.handle_na,
        );
    }
}
