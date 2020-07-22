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

/**
 * By default, no directories are stripped.
*/
#[test]
fn default_no_strip() {
    let directory = TempDir::new("single_file").unwrap();
    let dir = directory.into_path();
    let archive = dir.join("archive.zip");

    let mut writer = create_archive(&archive);
    zip_single_file("foo/bar.txt", b"Hello World", &mut writer);
    zip_single_file("foo/baz.txt", b"Hello World", &mut writer);
    writer.finish().unwrap();
    Unzipper::new(File::open(archive).unwrap(), &dir)
        .unzip()
        .unwrap();

    is_dir(dir.join("foo"));
    non_empty_file(dir.join("foo").join("bar.txt"));
    non_empty_file(dir.join("foo").join("baz.txt"));
}

/**
 * Nested contents are moved to the top-level if a directory is stripped.
*/
#[test]
fn strip_single_dir_contents() {
    let directory = TempDir::new("single_file").unwrap();
    let dir = directory.into_path();
    let archive = dir.join("archive.zip");

    let mut writer = create_archive(&archive);
    zip_single_file("foo/bar.txt", b"Hello World", &mut writer);
    zip_single_file("foo/baz.txt", b"Hello World", &mut writer);
    writer.finish().unwrap();
    Unzipper::new(File::open(archive).unwrap(), &dir)
        .strip_components(1)
        .unzip()
        .unwrap();

    non_empty_file(dir.join("bar.txt"));
    non_empty_file(dir.join("baz.txt"));
}

/**
 * Nested directories remain if only one directory is stripped.
*/
#[test]
fn strip_single_dir_subdir() {
    let directory = TempDir::new("single_file").unwrap();
    let dir = directory.into_path();
    let archive = dir.join("archive.zip");

    let mut writer = create_archive(&archive);
    zip_single_file("foo/bar/baz.txt", b"Hello World", &mut writer);
    writer.finish().unwrap();
    Unzipper::new(File::open(archive).unwrap(), &dir)
        .strip_components(1)
        .unzip()
        .unwrap();

    is_dir(dir.join("bar"));
    non_empty_file(dir.join("bar").join("baz.txt"));
}

/**
 * Multiple levels can be stripped.
*/
#[test]
fn strip_multiple_dirs() {
    let directory = TempDir::new("single_file").unwrap();
    let dir = directory.into_path();
    let archive = dir.join("archive.zip");

    let mut writer = create_archive(&archive);
    zip_single_file("foo/bar/baz.txt", b"Hello World", &mut writer);
    writer.finish().unwrap();
    Unzipper::new(File::open(archive).unwrap(), &dir)
        .strip_components(2)
        .unzip()
        .unwrap();

    non_empty_file(dir.join("baz.txt"));
}

/**
 * Two directories will be merged if they are both stripped.
*/
#[test]
fn strip_merge_dirs() {
    let directory = TempDir::new("single_file").unwrap();
    let dir = directory.into_path();
    let archive = dir.join("archive.zip");

    let mut writer = create_archive(&archive);
    zip_single_file("foo1/bar.txt", b"Hello World", &mut writer);
    zip_single_file("foo2/baz.txt", b"Hello World", &mut writer);
    writer.finish().unwrap();
    Unzipper::new(File::open(archive).unwrap(), &dir)
        .strip_components(1)
        .unzip()
        .unwrap();

    non_empty_file(dir.join("bar.txt"));
    non_empty_file(dir.join("baz.txt"));
}