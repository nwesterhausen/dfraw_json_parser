use std::path::Path;

use self::raws::info::DFInfoFile;
use self::raws::RawModuleLocation;
use self::util::get_parent_dir_name;

mod conversion;
pub mod json_conversion;
mod parsing;
mod parsing_bits;
pub mod raws;
mod reader;
mod refs;
pub mod util;

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
    let serializable_vec = parsing::parse_raw_module_into_serializable(root_path);

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
    let parent_dir = get_parent_dir_name(raw_file);
    let location = RawModuleLocation::Unknown;

    let info_text_file = DFInfoFile::new("manual-origin", location, &parent_dir);

    let serializable_vec =
        parsing::parse_raws_from_single_file_into_serializable(raw_file, &info_text_file);

    format!("[{}]", util::stringify_raw_vec(serializable_vec).join(","))
}

/// It takes a path to a info.txt file, and returns a `DFInfoFile` for it
///
/// Arguments:
///
/// * `input_path`: The path to the file to be parsed.
pub fn parse_info_file<P: AsRef<Path>>(input_path: &P) -> raws::info::DFInfoFile {
    reader::info_file::parse(input_path)
}

/// It reads the `info.txt` file in a given directory, and returns a struct containing the information
/// in that file
///
/// Arguments:
///
/// * `root_path`: The path to the directory containing the raws.
///
/// Returns:
///
/// `DFInfoFile`
pub fn parse_info_file_from_module_directory<P: AsRef<Path>>(root_path: &P) -> DFInfoFile {
    //1. Get information from the info.txt file
    if !root_path.as_ref().exists() {
        log::error!(
            "Provided directory to parse raws does not exist: {:?}",
            root_path.as_ref()
        );
        return DFInfoFile::empty();
    }
    if !root_path.as_ref().is_dir() {
        log::error!(
            "Provided 'directory' to parse is not actually a directory! {:?}",
            root_path.as_ref()
        );
        return DFInfoFile::empty();
    }

    // Check for info.txt
    let info_txt_path = root_path.as_ref().join("info.txt");
    if !info_txt_path.exists() {
        let Some(dir_name) = root_path.as_ref().file_name() else {
            log::error!("Error reading module dir {:?}", root_path.as_ref());
            return DFInfoFile::empty();
        };
        let dir_name_str = dir_name.to_str().unwrap_or("");

        if !(dir_name_str.eq("mod_upload")
            || dir_name_str.eq("examples and notes")
            || dir_name_str.eq("interaction examples"))
        {
            log::error!(
                "No info.txt as expected in {:?}. Is this DF 50.xx?",
                root_path.as_ref().file_name().unwrap_or_default()
            );
            return DFInfoFile::empty();
        }

        log::error!(
            "info_txt_path doesn't exist here: {}",
            info_txt_path.display()
        );
    }

    parse_info_file(&info_txt_path)
}
