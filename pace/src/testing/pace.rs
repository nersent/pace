use std::path::{Path, PathBuf};

pub fn format_pace_fixture_path(path: &str) -> PathBuf {
    let mut normalized_path = Path::new("fixtures").join(path);
    let test_mode = std::env::var("NEXTEST").is_ok();

    if test_mode {
        normalized_path = Path::new("../").join(normalized_path);
    }

    return normalized_path;
}
