extern crate zip;

use std::io::Write;
use std::fs::File;
use std::path::PathBuf;
use zip::ZipWriter;

pub fn create_archive(path: &PathBuf) -> ZipWriter<File> {
    let archive_file = File::create(path).unwrap();
    zip::ZipWriter::new(archive_file)
}

pub fn zip_single_file(filename: &str, content: &[u8], archive_file: &mut ZipWriter<File>) {
    let options = zip::write::FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored);
    archive_file.start_file(filename, options).unwrap();
    archive_file.write(content).unwrap();
}