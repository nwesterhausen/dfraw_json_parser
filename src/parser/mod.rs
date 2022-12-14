use serde_json::to_string;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use walkdir::WalkDir;

mod conversion;
mod parsing;
pub mod raws;
mod reader;

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
    turn_vec_into_json_string(parse_directory(raws_directory, info_text_file))
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
    save_vec_to_json_file(
        parse_directory(raws_directory, info_text_file),
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
/// A vector of DFCreature objects.
fn parse_directory(
    raws_directory: &Path,
    info_text_file: &raws::info::DFInfoFile,
) -> Vec<raws::creature::DFCreature> {
    let mut creatures: Vec<raws::creature::DFCreature> = Vec::new();

    // Read all the files in the directory, selectively parse the .txt files
    for entry in WalkDir::new(raws_directory)
        .into_iter()
        .filter_map(std::result::Result::ok)
    {
        let f_name = entry.file_name().to_string_lossy();

        if f_name.ends_with(".txt") {
            let entry_path = entry.path().to_string_lossy().to_string();
            log::debug!("parsing {}", &entry_path);
            creatures.append(&mut reader::parse_file(&entry_path, info_text_file));
        }
    }
    log::debug!(
        "{} creatures parsed from directory {:?}",
        creatures.len(),
        &raws_directory
    );
    creatures
}

/// It takes a vector of Creature raws, converts them to JSON, and returns a string of that JSON
///
/// Arguments:
///
/// * `creature_vec`: Vec<raws::creature::DFCreature>
///
/// Returns:
///
/// A JSON string
fn turn_vec_into_json_string(creature_vec: Vec<raws::creature::DFCreature>) -> String {
    // Return an empty string if there is no content to convert
    if creature_vec.len() == 0 {
        return String::new();
    }

    // Convert the content to JSON and squish between array brackets
    let mut owned_string: String = "[".to_owned();
    owned_string.push_str(stringify_raw_vec(creature_vec).join(",").as_str());
    owned_string.push(']');
    log::trace!("JSON String is {} characters", owned_string.len());

    owned_string
}

fn save_vec_to_json_file(v: Vec<raws::creature::DFCreature>, out_directory: &Path) {
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

    match write!(stream, "{}", stringify_raw_vec(v).join(",")) {
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

fn stringify_raw_vec(raws: Vec<raws::creature::DFCreature>) -> Vec<String> {
    let mut results: Vec<String> = Vec::new();
    if raws.is_empty() {
        return results;
    }
    log::debug!("{} creatures being serialized", raws.len());
    for creature in raws {
        match to_string(&conversion::WebCreature::from(&creature)) {
            Ok(s) => {
                results.push(s.to_string());
            }
            Err(e) => {
                log::error!("Failure to serialize creature\n{}", e);
            }
        }
    }
    results
}
