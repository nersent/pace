use crate::utils::array::{find_max_index, find_min_index};

pub fn compute_highest_bars(series: &[Option<f64>], length: usize) -> Option<i32> {
    let index = find_max_index(series);
    return Some(-(length.abs_diff(index).abs_diff(1) as i32));
}

pub fn compute_lowest_bars(series: &[Option<f64>], length: usize) -> Option<i32> {
    let index = find_min_index(series);
    return Some(-(length.abs_diff(index).abs_diff(1) as i32));
}

pub fn compute_highest(series: &[Option<f64>]) -> Option<f64> {
    let index = find_max_index(series);
    return series[index];
}

pub fn compute_lowest(series: &[Option<f64>]) -> Option<f64> {
    let index = find_min_index(series);
    return series[index];
}
