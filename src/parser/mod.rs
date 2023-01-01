use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use walkdir::WalkDir;

use self::json_conversion::TypedJsonSerializable;
use self::raws::info::DFInfoFile;

mod json_conversion;
mod parsing;
pub mod raws;
mod reader;
mod refs;

/// Given the raws directory and raw module info, returns a JSON string of parsed raw info.
///
/// Arguments:
///
/// * `raws_directory`: The directory where the raws are located.
/// * `raws_module_identifier`: The name of the raws file, e.g. "creatures"
/// * `raws_module_version`: The version of the raws you're parsing.
///
/// Returns:
///
/// A String
pub fn parse_directory_to_json_string(
    raws_directory: &Path,
    info_text_file: &raws::info::DFInfoFile,
) -> String {
    let json_string_vec = parse_raws_to_json(raws_directory, info_text_file);

    // Return an empty string if there is no content to convert
    if json_string_vec.len() == 0 {
        return String::new();
    }

    // Convert the content to JSON and squish between array brackets
    let mut owned_string: String = "[".to_owned();
    owned_string.push_str(json_string_vec.join(",").as_str());
    owned_string.push(']');
    log::trace!("JSON String is {} characters", owned_string.len());

    owned_string
}

/// It takes a directory of raws, parses them, and saves the result to out.json in `out_directory`
///
/// Arguments:
///
/// * `raws_directory`: The directory where the raws are located.
/// * `raws_module_identifier`: The name of the raws module
/// * `raws_module_version`: The version of the raws module
/// * `out_directory`: The directory to save the JSON file to.
pub fn parse_directory_to_json_file(
    raws_directory: &Path,
    info_text_file: &raws::info::DFInfoFile,
    out_directory: &Path,
) {
    save_string_vec_to_json_file(
        parse_raws_to_json(raws_directory, info_text_file),
        out_directory,
    );
}

/// Given the raws directory and the raw module details, will parse the raw files and return a
/// vector of Creature structs.
///
/// Arguments:
///
/// * `raws_directory`: The path to the directory containing the raws files.
/// * `raws_module_identifier`: The name of the raws, e.g. "Dwarf Fortress"
/// * `raws_module_version`: The version of the raws you're parsing.
///
/// Returns:
///
/// A vector serialized JSON parsed from the files.
fn parse_raws_to_json(
    raws_directory: &Path,
    info_text_file: &raws::info::DFInfoFile,
) -> Vec<String> {
    let caller = "parse_directory";
    let mut parsed_raws: Vec<String> = Vec::new();

    // Read all the files in the directory, selectively parse the .txt files
    for entry in WalkDir::new(raws_directory)
        .into_iter()
        .filter_map(std::result::Result::ok)
    {
        let f_name = entry.file_name().to_string_lossy();

        if f_name.ends_with(".txt") {
            let entry_path = entry.path();

            match reader::read_raw_file_type(entry_path) {
                reader::RawObjectKind::Creature => {
                    log::debug!("parsing {}", entry_path.display());
                    let creature_raw_vec = reader::parse_creature_file(&entry_path, info_text_file);
                    parsed_raws.extend(stringify_raw_vec(creature_raw_vec));
                }
                _ => log::trace!("{} - skipping {}", caller, entry_path.display()),
            }
        }
    }
    log::debug!(
        "{} raws parsed from directory {:?}",
        parsed_raws.len(),
        &raws_directory
    );
    parsed_raws
}

pub fn read_single_raw_file(raw_file: &Path) -> String {
    let info_text_file = DFInfoFile::new("manual-origin", "user-specified");

    match reader::read_raw_file_type(raw_file) {
        reader::RawObjectKind::Creature => {
            log::info!("Parsing creature raws from {}", raw_file.display());
            let creature_raw_vec = reader::parse_creature_file(&raw_file, &info_text_file);
            return format!("[{}]", stringify_raw_vec(creature_raw_vec).join(","));
        }
        reader::RawObjectKind::Plant => {
            log::info!("Parsing plant raws from {}", raw_file.display());
            let plant_raw_vec = reader::parse_plant_file(&raw_file, &info_text_file);
            return format!("[{}]", stringify_raw_vec(plant_raw_vec).join(","));
        }
        _ => {
            log::warn!("Unknown raw type or failure to parse it.");
            return String::new();
        }
    }
}

fn save_string_vec_to_json_file(string_vec: Vec<String>, out_directory: &Path) {
    // The destination file is out.json inside the out_directory
    let out_filepath = out_directory.join("out.json");
    let out_file = match File::create(&out_filepath.as_path()) {
        Ok(f) => f,
        Err(e) => {
            log::error!("Unable to open out.json for writing \n{:?}", e);
            return;
        }
    };

    let mut stream = BufWriter::new(out_file);
    let write_error = &format!("Unable to write to {}", out_filepath.to_string_lossy());
    match write!(stream, "[") {
        Ok(_x) => (),
        Err(e) => {
            log::error!("{}\n{:?}", write_error, e);
            return;
        }
    };

    match write!(stream, "{}", string_vec.join(",")) {
        Ok(_x) => (),
        Err(e) => {
            log::error!("{}\n{:?}", write_error, e);
            return;
        }
    };
    match stream.flush() {
        Ok(_x) => (),
        Err(e) => {
            log::error!("{}\n{:?}", write_error, e);
            return;
        }
    };

    match write!(stream, "]") {
        Ok(_x) => (),
        Err(e) => {
            log::error!("{}\n{:?}", write_error, e);
            return;
        }
    };
    match stream.flush() {
        Ok(_x) => (),
        Err(e) => {
            log::error!("{}\n{:?}", write_error, e);
        }
    };
}

pub fn parse_info_file(input_path: &Path, sourced_dir: &str) -> raws::info::DFInfoFile {
    reader::parse_dfraw_module_info_file(input_path, sourced_dir)
}

pub fn parse_info_file_to_json_string(input_path: &Path, sourced_dir: &str) -> String {
    let info_file = parse_info_file(input_path, sourced_dir);
    match info_file.to_typed_json_string() {
        Ok(s) => {
            return s.to_string();
        }
        Err(e) => {
            log::error!("Failure to serialize parsed raw data\n{}", e);
            return "".to_owned();
        }
    }
}

fn stringify_raw_vec(
    serializable_vec: Vec<impl json_conversion::TypedJsonSerializable>,
) -> Vec<String> {
    let mut results: Vec<String> = Vec::new();
    if serializable_vec.is_empty() {
        return results;
    }

    for raw_object in serializable_vec {
        match raw_object.to_typed_json_string() {
            Ok(s) => {
                results.push(s.to_string());
            }
            Err(e) => {
                log::error!("Failure to serialize parsed raw data\n{}", e);
            }
        }
    }
    results
}
