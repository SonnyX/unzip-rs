use std::io;
use std::path::{Path, PathBuf};
use transformation_pipeline::{TransformationStage, StageActions, StageResult};
use transformation_metadata::ExtractionMetadata;

pub struct StripComponents {

    num: u8,

}

impl StripComponents {

    pub fn new(num: u8) -> StripComponents {
        StripComponents {
            num: num,
        }
    }

}

impl TransformationStage<ExtractionMetadata> for StripComponents {
    fn run(&self, previous: ExtractionMetadata) -> StageResult<ExtractionMetadata> {
        if self.num < 1 {
            return Ok(StageActions::Skip);
        }

        let last_filename: String = previous.filename.clone();
        let path: &Path = Path::new(&last_filename);

        if path.components().count() < self.num.into() {
            return Ok(StageActions::Finish(ExtractionMetadata {
                extract: false,

                comment: previous.comment,
                compressed_size: previous.compressed_size,
                uncompressed_size: previous.uncompressed_size,
                crc32: previous.crc32,
                data_start: previous.data_start,
                filename: previous.filename,
                last_modified: previous.last_modified,
                unix_mode: previous.unix_mode,
            }));
        }

        let start_str: String = "".to_owned();
        let start: &Path = Path::new(&start_str);
        let mut output: PathBuf = start.join(".");
        path
            .components()
            .skip(self.num.into())
            .map(|comp| comp.as_os_str())
            .for_each(|comp| output = output.join(comp));

        let joined = output.as_os_str().to_str();
        if joined.is_none() {
            return Err(io::Error::new(io::ErrorKind::Other, "Couldn't join stripped string."));
        }

        Ok(StageActions::Next(ExtractionMetadata {
            filename: joined.unwrap().to_owned(),

            extract: previous.extract,
            comment: previous.comment,
            compressed_size: previous.compressed_size,
            uncompressed_size: previous.uncompressed_size,
            crc32: previous.crc32,
            data_start: previous.data_start,
            last_modified: previous.last_modified,
            unix_mode: previous.unix_mode,
        }))
    }
}