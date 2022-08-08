use std::path::Path;

mod parser;

pub fn parse_directory_to_json_string(raws_directory: &str) -> String {
    parser::parse_directory_to_json_string(raws_directory)
}

pub fn parse_directory_to_json_file(raws_directory: &str, out_directory: &Path) {
    parser::parse_directory_to_json_file(raws_directory, out_directory);
}
