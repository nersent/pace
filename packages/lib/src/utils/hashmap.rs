use std::collections::HashMap;

pub fn with_prefix(
    map: HashMap<String, Option<f64>>,
    prefix: &str,
) -> HashMap<String, Option<f64>> {
    let mut new_map = HashMap::new();
    for (key, value) in map {
        new_map.insert(format!("{}{}", prefix, key), value);
    }
    return new_map;
}
