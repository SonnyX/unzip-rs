extern crate zip;

use std::fs;
use std::io;
use std::io::prelude::Read;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct UnzipperStats {
    pub dirs: u16,
    pub files: u16,
}

type UnzipperResult = Result<UnzipperStats, io::Error>;

pub struct Unzipper<R: Read + io::Seek, O: AsRef<Path>> {
    source: R,
    outdir: O,
    strip_components: u8,
}

impl<R: Read + io::Seek, O: AsRef<Path>> Unzipper<R, O> {
    pub fn new(reader: R, output: O) -> Unzipper<R, O> {
        Unzipper {
            source: reader,
            outdir: output,
            strip_components: 0,
        }
    }

    pub fn strip_components(mut self, num: u8) -> Unzipper<R, O> {
        self.strip_components = num;
        self
    }

    pub fn unzip(self) -> UnzipperResult {
        let mut zip = zip::ZipArchive::new(self.source)?;

        let mut stats = UnzipperStats { dirs: 0, files: 0 };

        for i in 0..zip.len() {
            let mut entry = zip.by_index(i).unwrap();
            let mut filename = PathBuf::new();
            let name = entry.name();

            if name.contains("\\") {
                let dir_entry = name.split("\\").collect::<Vec<&str>>().join("/");

                filename.push(dir_entry);
            } else {
                filename.push(entry.name());
            }

            if self.strip_components > 0 {
                if filename.components().count() < self.strip_components.into() {
                    continue;
                }
                let mut output: PathBuf = PathBuf::new();
                output.push(".");
                filename
                    .components()
                    .skip(self.strip_components.into())
                    .map(|comp| comp.as_os_str())
                    .for_each(|comp| output = output.join(comp));

                filename = output;
                if filename.to_str().is_none() {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        "Couldn't join stripped string.",
                    ));
                }
            }

            let outdir = Path::new(self.outdir.as_ref()).join(filename);

            if entry.is_dir() {
                fs::create_dir_all(outdir)?;
                stats.dirs = stats.dirs + 1;
                continue;
            }
            if let Some(parent_dir) = outdir.as_path().parent() {
                fs::create_dir_all(&parent_dir)?;
            }

            let mut dest = bin_open_options()
                .write(true)
                .create_new(true)
                .open(outdir)?;
            io::copy(&mut entry, &mut dest)?;

            stats.files = stats.files + 1;
        }

        #[cfg(unix)]
        fn bin_open_options() -> fs::OpenOptions {
            use std::os::unix::fs::OpenOptionsExt;

            let mut opts = fs::OpenOptions::new();
            opts.mode(0o755);
            opts
        }

        #[cfg(not(unix))]
        fn bin_open_options() -> fs::OpenOptions {
            fs::OpenOptions::new()
        }
        Ok(stats)
    }
}
