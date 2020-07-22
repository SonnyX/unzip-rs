extern crate zip;
extern crate time;
extern crate transformation_pipeline;

mod transformation_metadata;

use std::io;
use std::io::prelude::Read;
use std::fs;
use std::path::{Path, PathBuf};
use transformation_pipeline::TransformationPipeline;
use transformation_metadata::ExtractionMetadata;

mod strip_components;
use strip_components::StripComponents;

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
        let mut archive = zip::ZipArchive::new(self.source)?;
        let outdir: &Path = Path::new(self.outdir.as_ref());

        let mut stats = UnzipperStats {
            dirs: 0,
            files: 0,
        };

        let pipeline: TransformationPipeline<ExtractionMetadata> = TransformationPipeline::new(vec![
            Box::new(StripComponents::new(self.strip_components)),
        ]);

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;

            let metadata = pipeline.run(ExtractionMetadata {
                extract: true,
                filename: file.name().to_owned(),
                comment: file.comment().to_owned(),
                compressed_size: file.compressed_size(),
                uncompressed_size: file.size(),
                crc32: file.crc32(),
                data_start: file.data_start(),
                last_modified: file.last_modified(),
                unix_mode: file.unix_mode(),
            })?;

            if !metadata.extract {
                continue;
            }

            let outpath: PathBuf = outdir.join(metadata.filename);

            if let Some(parent_dir) = outpath.as_path().parent() {
                fs::create_dir_all(&parent_dir)?;
            }

            if (&*file.name()).ends_with('/') {
                stats.dirs = stats.dirs + 1;
                continue;
            }
            let mut outfile = fs::File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;

            // TODO: Handle unix_mode, last_modified

            stats.files = stats.files + 1;
        }
        Ok(stats)
    }
}