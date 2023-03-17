use std::{ffi::OsStr, path::Path};

pub fn get_filename_extension(path: &Path) -> Option<&str> {
    return path.extension().and_then(OsStr::to_str);
}

pub fn get_filename(path: &Path) -> Option<&str> {
    return path.file_stem().and_then(OsStr::to_str);
}

pub fn ensure_dir(path: &Path) {
    if !path.exists() {
        std::fs::create_dir_all(path).unwrap();
    }
}
