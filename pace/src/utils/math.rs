pub fn round_to_min_tick(value: f64, min_tick: f64) -> f64 {
    if value.is_nan() {
        return 0.0;
    }
    if min_tick.is_nan() {
        return value;
    }
    return (value / min_tick).round() * min_tick;
}
