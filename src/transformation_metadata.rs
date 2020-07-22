use time::Tm;
use std::clone::Clone;

pub struct ExtractionMetadata {

    // Fields that can't effect output:
    // (aka don't modify them unless you really need to change downstream behavior)
    pub comment: String,

    pub compressed_size: u64,

    pub uncompressed_size: u64,

    pub crc32: u32,

    pub data_start: u64,

    // Fields used by output:
    // (aka most likely to be modified)

    pub extract: bool,

    pub filename: String,

    pub last_modified: Tm,

    pub unix_mode: Option<u32>,

}

impl Clone for ExtractionMetadata {
    fn clone(&self) -> Self {
        ExtractionMetadata {
            comment: self.comment.clone(),
            compressed_size: self.compressed_size.clone(),
            uncompressed_size: self.uncompressed_size.clone(),
            crc32: self.crc32.clone(),
            data_start: self.data_start.clone(),
            extract: self.extract.clone(),
            filename: self.filename.clone(),
            last_modified: self.last_modified.clone(),
            unix_mode: self.unix_mode.clone(),
        }
    }
}