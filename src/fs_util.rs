use std::fs;
use std::path::Path;

pub fn is_file(path: &str) -> bool {
    fs::metadata(path).is_ok_and(|stat| stat.is_file())
}

pub fn is_dir(path: &str) -> bool {
    fs::metadata(path).is_ok_and(|stat| stat.is_dir())
}

pub fn is_dir_path(path: &Path) -> bool {
    path.is_dir()
}
