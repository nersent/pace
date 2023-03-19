pub fn with_suffix(suffix: &str) -> impl Fn(f64) -> String {
    let suffix = suffix.to_string();
    move |value| format!("{:0.2}{}", value, suffix)
}
