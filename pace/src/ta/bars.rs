use crate::statistics::math::{find_max_index, find_min_index};

/// Highest value offset for a given number of bars back.
///
/// Similar to PineScript `ta.highestbars(src, length)`, but `src` array requires to be truncated to the length.
pub fn highest_bars(series: &[Option<f64>], length: usize) -> Option<i32> {
    let index = find_max_index(series);
    return Some(-(length.abs_diff(index).abs_diff(1) as i32));
}

/// Lowest value offset for a given number of bars back.
///
/// Similar to PineScript `ta.lowestbars(src, length)`, but `src` array requires to be truncated to the length.
pub fn lowest_bars(series: &[Option<f64>], length: usize) -> Option<i32> {
    let index = find_min_index(series);
    return Some(-(length.abs_diff(index).abs_diff(1) as i32));
}

/// Highest value for a given number of bars back.
///
/// Similar to PineScript `ta.highest(src, length)`, but `src` array requires to be truncated to the length.
pub fn highest(series: &[Option<f64>]) -> Option<f64> {
    let index = find_max_index(series);
    return series[index];
}

/// Lowest value for a given number of bars back.
///
/// Similar to PineScript `ta.lowest(src, length)`, but `src` array requires to be truncated to the length.
pub fn lowest(series: &[Option<f64>]) -> Option<f64> {
    let index = find_min_index(series);
    return series[index];
}
