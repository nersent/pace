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
