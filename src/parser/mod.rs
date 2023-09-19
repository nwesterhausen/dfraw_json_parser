use std::path::Path;

use self::{mod_info_file::ModuleInfoFile, raws::DFRaw};

mod biomes;
mod graphics;
pub(crate) mod mod_info_file;
mod names;
pub(crate) mod raw_locations;
mod raw_object_kind;
pub(crate) mod raws;
mod reader;
mod refs;
mod tags;

pub(crate) fn parse_raw_module_to_json_string<P: AsRef<Path>>(raw_module_path: &P) -> String {
    String::from("Not implemented")
}
pub(crate) fn parse_info_file_from_module_directory<P: AsRef<Path>>(
    raw_module_directory: &P,
) -> String {
    String::from("Not implemented")
}
pub(crate) fn parse_single_raw_file_to_json_string<P: AsRef<Path>>(raw_file: &P) -> String {
    String::from("Not implemented")
}

pub(crate) fn parse_raws_from_single_file<P: AsRef<Path>>(entry_path: &P) -> Vec<DFRaw> {
    reader::parse_file::parse_raw_file(entry_path)
}

pub(crate) fn parse_raw_module<P: AsRef<Path>>(raw_module_path: &P) -> Vec<DFRaw> {
    Vec::new()
}
