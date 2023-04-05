use super::float::OptionFloatUtils;

pub fn map_option_f64_list_to_f64(list: &[Option<f64>]) -> Vec<f64> {
    return list.iter().map(|x| x.unwrap_nan()).collect();
}
