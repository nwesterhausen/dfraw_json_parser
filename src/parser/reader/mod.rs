use std::path::Path;

use crate::{options::ParserOptions, parser::info_txt::ModuleInfoFile, parser::raws::RawObject};

mod header;
mod parsable_types;
mod parse_file;

pub use header::read_raw_file_type;
pub use parsable_types::PARSABLE_OBJECT_TYPES;
pub use parse_file::parse_raw_file;
pub use parse_file::parse_raw_file_with_info;

pub fn parse_info_file_from_file_path<P: AsRef<Path>>(raw_file_path: &P) -> ModuleInfoFile {
    ModuleInfoFile::parse(&raw_file_path.as_ref())
}

pub fn parse_raws_from_single_file<P: AsRef<Path>>(
    entry_path: &P,
    options: &ParserOptions,
) -> Vec<Box<dyn RawObject>> {
    parse_file::parse_raw_file(entry_path, options)
}
