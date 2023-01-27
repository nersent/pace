pub struct TrueRange {}

impl TrueRange {
    pub fn tr(
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
}
