use crate::utils::array::{find_max_index, find_min_index};

pub struct Bars {}

impl Bars {
    pub fn highest_bars(series: &[Option<f64>], length: usize) -> Option<i32> {
        let index = find_max_index(series);
        return Some(-(length.abs_diff(index).abs_diff(1) as i32));
    }

    pub fn lowest_bars(series: &[Option<f64>], length: usize) -> Option<i32> {
        let index = find_min_index(series);
        return Some(-(length.abs_diff(index).abs_diff(1) as i32));
    }

    pub fn highest(series: &[Option<f64>], length: usize) -> Option<f64> {
        let index = find_max_index(series);
        return series[index];
    }

    pub fn lowest(series: &[Option<f64>], length: usize) -> Option<f64> {
        let index = find_min_index(series);
        return series[index];
    }
}
