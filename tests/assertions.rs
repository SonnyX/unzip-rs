use std::fs;
use std::path::PathBuf;

pub fn non_empty_file(path: PathBuf) {
    let data = fs::metadata(path).unwrap();
    assert!(data.is_file());
    assert!(data.len() > 0);
}

pub fn is_dir(path: PathBuf) {
    let data = fs::metadata(path).unwrap();
    assert!(data.is_dir());
}