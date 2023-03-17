pub fn clip_value(value: f64, min: f64, max: f64) -> f64 {
    return f64::min(f64::max(value, min), max);
}

pub fn scale_value_up(value: f64, threshold: f64, max: f64) -> f64 {
    if value <= threshold {
        return 0.0;
    }

    let delta = value - threshold;
    let mean = max - threshold;

    return clip_value(delta / mean, 0.0, 1.0);
}

pub fn scale_value_down(value: f64, threshold: f64, min: f64) -> f64 {
    if value >= threshold {
        return 0.0;
    }

    let delta = value - threshold;
    let mean = min - threshold;
    return clip_value(delta / mean, 0.0, 1.0);
}

pub fn scale_value_centered(value: f64, mean: f64, min: f64, max: f64) -> f64 {
    let distance = (max - min) / 2.0;
    let abs_diff = (value - mean).abs();
    return clip_value(1.0 - abs_diff / distance, 0.0, 1.0);
}

pub fn scale_value_around_mean(value: f64, mean: f64) -> f64 {
    return (2.0 * value - mean) / mean;
}

pub fn scale_value_min_max(value: f64, min: f64, max: f64) -> f64 {
    let mut value = value;
    let mut min = min;
    let mut max = max;
    if min < 0.0 {
        let offset = min.abs();
        min = 0.0;
        max += offset;
        value += offset;
    }
    let mean = max - min;
    return scale_value_around_mean(value, mean);
}
