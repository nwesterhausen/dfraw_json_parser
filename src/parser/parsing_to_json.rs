use std::path::Path;

use crate::{parser::raws::info_txt::DFInfoFile, util};

use super::{raws, DFParser};

impl DFParser {
    /// Parse a directory of raws, and return a JSON string of the parsed raws.
    ///
    /// The first thing we do is check that the directory exists, and that it's a directory. If it doesn't
    /// exist, or it's not a directory, we return an empty string
    ///
    /// Arguments:
    ///
    /// * `root_path`: The path to the directory containing the raws.
    /// * `sourced_dir`: The directory the `root_path` was found under.
    ///
    /// Returns:
    ///
    /// A JSON string containing the raws for the given directory.
    pub fn parse_raw_module_to_json_string<P: AsRef<Path>>(root_path: &P) -> String {
        // Parse raws in the location
        let serializable_vec = DFParser::parse_raw_module_into_serializable(root_path);

        // Convert those raws into JSON strings
        let json_vec = util::stringify_raw_vec(serializable_vec);

        //3. Return the object array for this dfraw dir
        format!("[{}]", json_vec.join(","))
    }

    /// It takes a path to a raw file, parses it, and returns a string representation of the parsed raws
    ///
    /// Arguments:
    ///
    /// * `raw_file`: The path to the raw file to read.
    ///
    /// Returns:
    ///
    /// A string.
    pub fn parse_single_raw_file_to_json_string<P: AsRef<Path>>(raw_file: &P) -> String {
        let parent_dir = util::get_parent_dir_name(raw_file);
        let location = raws::RawModuleLocation::Unknown;

        let info_text_file = DFInfoFile::new("manual-origin", location, &parent_dir);

        let serializable_vec =
            DFParser::parse_raws_from_single_file_into_serializable(raw_file, &info_text_file);

        format!("[{}]", util::stringify_raw_vec(serializable_vec).join(","))
    }

    /// It takes a path to a raw file, parses it, and returns a string representation of the parsed raws
    ///
    /// Arguments:
    ///
    /// * `raw_file`: The path to the raw file to read.
    ///
    /// Returns:
    ///
    /// A string.
    pub fn parse_single_graphics_raw_file_to_json_string<P: AsRef<Path>>(raw_file: &P) -> String {
        let parent_dir = util::get_parent_dir_name(raw_file);
        let location = raws::RawModuleLocation::Unknown;

        let info_text_file = DFInfoFile::new("manual-origin", location, &parent_dir);

        let serializable_vec = DFParser::parse_graphics_raws_from_single_file_into_serializable(
            raw_file,
            &info_text_file,
        );

        format!("[{}]", util::stringify_raw_vec(serializable_vec).join(","))
    }
    /// Parse a directory of raws, and return a JSON string of the parsed raws.
    ///
    /// The first thing we do is check that the directory exists, and that it's a directory. If it doesn't
    /// exist, or it's not a directory, we return an empty string
    ///
    /// Arguments:
    ///
    /// * `root_path`: The path to the directory containing the raws.
    /// * `sourced_dir`: The directory the `root_path` was found under.
    ///
    /// Returns:
    ///
    /// A JSON string containing the raws for the given directory.
    pub fn parse_graphics_raw_module_to_json_string<P: AsRef<Path>>(root_path: &P) -> String {
        // Parse raws in the location
        let serializable_vec = DFParser::parse_graphics_raw_module_into_serializable(root_path);

        // Convert those raws into JSON strings
        let json_vec = util::stringify_raw_vec(serializable_vec);

        //3. Return the object array for this dfraw dir
        format!("[{}]", json_vec.join(","))
    }
}
