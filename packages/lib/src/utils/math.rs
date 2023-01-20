pub fn compare_floats(a: f64, b: f64, precision: f64) -> bool {
    return (a - b).abs() < precision;
}

pub fn find_max_index(arr: &[Option<f64>]) -> usize {
    assert!(!arr.is_empty(), "Array must have at least one element");

    let mut max_value: Option<f64> = None;
    let mut max_value_index: Option<usize> = None;

    for i in 0..arr.len() {
        let value = arr[i];
        match (max_value, value) {
            (None, Some(value)) => {
                max_value = Some(value);
                max_value_index = Some(i);
            }
            (Some(_max_value), Some(_value)) if _value > _max_value => {
                max_value = value;
                max_value_index = Some(i);
            }
            _ => {}
        }
    }
    if max_value_index.is_none() {
        panic!("max_value_index is None");
    }

    return max_value_index.unwrap();
}

pub fn find_min_index(arr: &[Option<f64>]) -> usize {
    assert!(!arr.is_empty(), "Array must have at least one element");

    let mut max_value: Option<f64> = None;
    let mut max_value_index: Option<usize> = None;

    for i in 0..arr.len() {
        let value = arr[i];
        match (max_value, value) {
            (None, Some(value)) => {
                max_value = Some(value);
                max_value_index = Some(i);
            }
            (Some(_max_value), Some(_value)) if _value < _max_value => {
                max_value = value;
                max_value_index = Some(i);
            }
            _ => {}
        }
    }
    if max_value_index.is_none() {
        panic!("max_value_index is None");
    }

    return max_value_index.unwrap();
}

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
    let mean = max - min;
    return scale_value_around_mean(value, mean);
}
