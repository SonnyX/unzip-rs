extern crate zip;
extern crate unzip;
extern crate temporary;

use std::fs::File;
use self::temporary::Directory as TempDir;

use unzip::Unzipper;

mod assertions;
use assertions::{non_empty_file, is_dir};

mod utils;
use utils::{create_archive, zip_single_file};

#[test]
fn unpacks_single_file() {
    let directory = TempDir::new("single_file").unwrap();
    let dir = directory.into_path();
    let archive = dir.join("archive.zip");

    let mut writer = create_archive(&archive);
    zip_single_file("foo.txt", b"Hello World", &mut writer);
    writer.finish().unwrap();
    Unzipper::new(File::open(archive).unwrap(), &dir).unzip().unwrap();

    non_empty_file(dir.join("foo.txt"));
}

#[test]
fn unpacks_multiple_files() {
    let directory = TempDir::new("single_file").unwrap();
    let dir = directory.into_path();
    let archive = dir.join("archive.zip");

    let mut writer = create_archive(&archive);
    zip_single_file("foo.txt", b"Hello World", &mut writer);
    zip_single_file("bar.txt", b"Hello World", &mut writer);
    writer.finish().unwrap();
    Unzipper::new(File::open(archive).unwrap(), &dir).unzip().unwrap();

    non_empty_file(dir.join("foo.txt"));
    non_empty_file(dir.join("bar.txt"));
}

#[test]
fn creates_subdirectories() {
    let directory = TempDir::new("single_file").unwrap();
    let dir = directory.into_path();
    let archive = dir.join("archive.zip");

    let mut writer = create_archive(&archive);
    zip_single_file("foo/bar.txt", b"Hello World", &mut writer);
    writer.finish().unwrap();
    Unzipper::new(File::open(archive).unwrap(), &dir).unzip().unwrap();

    is_dir(dir.join("foo"));
}

#[test]
fn creates_files_in_subdirectories() {
    let directory = TempDir::new("single_file").unwrap();
    let dir = directory.into_path();
    let archive = dir.join("archive.zip");

    let mut writer = create_archive(&archive);
    zip_single_file("foo/bar.txt", b"Hello World", &mut writer);
    writer.finish().unwrap();
    Unzipper::new(File::open(archive).unwrap(), &dir).unzip().unwrap();

    non_empty_file(dir.join("foo/bar.txt"));
}

#[test]
fn unpacks_deep_directory() {
    let directory = TempDir::new("single_file").unwrap();
    let dir = directory.into_path();
    let archive = dir.join("archive.zip");

    let mut writer = create_archive(&archive);
    zip_single_file("foo/bar/baz.txt", b"Hello World", &mut writer);
    writer.finish().unwrap();
    Unzipper::new(File::open(archive).unwrap(), &dir).unzip().unwrap();

    is_dir(dir.join("foo"));
    is_dir(dir.join("foo/bar"));
    non_empty_file(dir.join("foo/bar/baz.txt"));
}