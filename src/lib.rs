//! dfraw_json_parser provides a way to turn raw files from Dwarf Fortress into JSON

use std::path::Path;

mod parser;

/// This will return JSON (in string format) of the raw files in raws_directory.
/// If you only want to parse a single file, you can pass the file's path as 'raws_directory'
pub fn parse_directory_to_json_string(raws_directory: &str) -> String {
    parser::parse_directory_to_json_string(raws_directory)
}

/// This will parse what raw files it finds at raws_directory and save the parsed JSON
/// to an 'out.json' file in the out_directory.
pub fn parse_directory_to_json_file(raws_directory: &str, out_directory: &Path) {
    parser::parse_directory_to_json_file(raws_directory, out_directory);
}
