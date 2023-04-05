pub fn find_max_index(arr: &[f64]) -> usize {
    assert!(!arr.is_empty(), "Array must have at least one element");

    let size = arr.len();

    let mut max_value_index: usize = size - 1;
    let mut max_value: f64 = arr[max_value_index];

    for i in (0..size).rev() {
        if arr[i].is_nan() {
            return max_value_index;
        }

        if arr[i] >= max_value || max_value.is_nan() {
            max_value = arr[i];
            max_value_index = i;
        }
    }

    return max_value_index;
}

pub fn find_min_index(arr: &[f64]) -> usize {
    assert!(!arr.is_empty(), "Array must have at least one element");

    let size = arr.len();

    let mut min_value_index: usize = size - 1;
    let mut min_value: f64 = arr[min_value_index];

    // iterate backwards to find the first non-NaN value
    for i in (0..size).rev() {
        if arr[i].is_nan() {
            return min_value_index;
        }

        if arr[i] <= min_value || min_value.is_nan() {
            min_value = arr[i];
            min_value_index = i;
        }
    }

    return min_value_index;

    // if size >= 2 && arr[size - 2].is_nan() && !arr[size - 1].is_nan() {
    //     return size - 1;
    // }

    // for i in 0..size {
    //     if !arr[i].is_nan() && arr[i] < min_value || min_value.is_nan() {
    //         min_value = arr[i];
    //         min_value_index = i;
    //     }
    // }

    // return min_value_index;
}
