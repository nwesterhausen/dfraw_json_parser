/*!
dfraw_json_parser provides a way to turn raw files from Dwarf Fortress into JSON.
It functions using a regular expression to break apart the tokens and then checks the
key in the token against a long list of ones it knows about. Any matches are utilized to
build its own representation of the raw before a final step where it dumps it to JSON.

Currently the only raws that are parsed are Creature raws.

Currently the JSON is returned as a string
*/

#![deny(clippy::pedantic)]

use parser::json_conversion::TypedJsonSerializable;
use parser::raws::RawModuleLocation;
use parser::util::subdirectories;
use std::path::{Path, PathBuf};
use walkdir::DirEntry;

#[cfg(feature = "tauri")]
extern crate tauri;

mod parser;

//Todo: add an option to apply any COPY_TAG_FROM and APPLY_CREATURE_VARIATION tags
//Todo: add an option to specify what kinds of raws to parse
//Todo: add a parameter for JSON_FILE name (currently hard-coded to out.json)

/// It takes a path to a DF raw module-containing directory, and returns a JSON array of all the raws
/// (from their own raw modules) in that directory
///
/// Arguments:
///
/// * `raw_module_location_path`: The path to the directory containing the raw modules. This has subdirectories
/// which contain an 'info.txt' file and raw objects or graphics.
///
/// Returns:
///
/// A string vec containing the JSON array for each parsed raws module
pub fn parse_module_location(raw_module_location_path: &Path) -> String {
    // Guard against invalid path
    if !raw_module_location_path.exists() {
        log::error!(
            "Provided module path for parsing doesn't exist!\n{}",
            raw_module_location_path.display()
        );
        return String::new();
    }
    if !raw_module_location_path.is_dir() {
        log::error!(
            "Raw module path needs to be a directory {}",
            raw_module_location_path.display()
        );
        return String::new();
    }

    //2. Get module location from provided path
    let module_location = RawModuleLocation::from_sourced_directory(
        &raw_module_location_path
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default(),
    );

    //3. Get list of all subdirectories
    let raw_module_iter: Vec<DirEntry> =
        subdirectories(PathBuf::from(raw_module_location_path)).unwrap_or(Vec::new());

    log::info!(
        "{num} raw modules located in {location:?}",
        num = raw_module_iter.len(),
        location = module_location
    );

    let mut all_json: Vec<String> = Vec::new();
    //4. Loop over all raw modules in the raw module directory
    for raw_module_directory in raw_module_iter {
        //2. Parse raws and dump JSON into array
        all_json.push(parse_raw_module(raw_module_directory.path()));
    }

    format!("[{}]", all_json.join(","))
}

/// It takes a path to a DF raw module-containing directory, and returns a JSON array of all the info.txt details
/// (from their own raw modules) in that directory
///
/// Arguments:
///
/// * `raw_module_location_path`: The path to the directory containing the raw modules. This has subdirectories
/// which contain an 'info.txt' file and raw objects or graphics.
///
/// Returns:
///
/// A string vec containing the JSON array for each parsed raws module's info
pub fn parse_info_txt_in_module_location(raw_module_location_path: &Path) -> String {
    // Guard against invalid path
    if !raw_module_location_path.exists() {
        log::error!(
            "Provided module path for parsing doesn't exist!\n{}",
            raw_module_location_path.display()
        );
        return String::new();
    }
    if !raw_module_location_path.is_dir() {
        log::error!(
            "Raw module path needs to be a directory {}",
            raw_module_location_path.display()
        );
        return String::new();
    }

    //2. Get module location from provided path
    let module_location = RawModuleLocation::from_sourced_directory(
        &raw_module_location_path
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default(),
    );

    //3. Get list of all subdirectories
    let raw_module_iter: Vec<DirEntry> =
        subdirectories(PathBuf::from(raw_module_location_path)).unwrap_or(Vec::new());

    log::info!(
        "{num} raw modules located in {location:?}",
        num = raw_module_iter.len(),
        location = module_location
    );

    let mut all_json: Vec<String> = Vec::new();
    //4. Loop over all raw modules in the raw module directory
    for raw_module_directory in raw_module_iter {
        //2. Parse raws and dump JSON into array
        all_json.push(parse_info_txt_from_raw_module(raw_module_directory.path()));
    }

    format!("[{}]", all_json.join(","))
}

/// Takes a path to a DF game directory, and returns a JSON array of all the raws in that directory
///
/// Arguments:
///
/// * `df_game_path`: The path to the game directory. This is the directory that contains the data,
/// mods, and gamelog.txt files.
///
/// Returns:
///
/// A string vec containing the JSON array for each parsed raws module
pub fn parse_game_raws(df_game_path: &str) -> Vec<String> {
    let mut all_json: Vec<String> = Vec::new();

    //1. "validate" folder is as expected
    let game_path = Path::new(df_game_path);
    // Guard against invalid path
    if !game_path.exists() {
        log::error!(
            "Provided game path for parsing doesn't exist!\n{}",
            df_game_path
        );
        return all_json;
    }
    if !game_path.is_dir() {
        log::error!("Game path needs to be a directory {}", df_game_path);
        return all_json;
    }

    // warn on no gamelog.txt
    if !game_path.join("gamelog.txt").exists() {
        log::warn!("Unable to find gamelog.txt in game directory. Is it valid?");
    }

    // Set file paths for vanilla raw modules, workshop mods and installed mods
    let data_path = game_path.join("data");
    let vanilla_path = data_path.join("vanilla");
    let installed_mods_path = data_path.join("installed_mods");
    let workshop_mods_path = game_path.join("mods");

    all_json.push(parse_module_location(&vanilla_path.as_path()));
    all_json.push(parse_module_location(&installed_mods_path.as_path()));
    all_json.push(parse_module_location(&workshop_mods_path.as_path()));

    let non_empty_json: Vec<String> = all_json
        .into_iter()
        .filter(|s| !String::is_empty(s))
        .collect();

    non_empty_json
}

/// Parse a directory of raws, and return a JSON string of the parsed raws.
///
/// The first thing we do is check that the directory exists, and that it's a directory. If it doesn't
/// exist, or it's not a directory, we return an empty string
///
/// Arguments:
///
/// * `root_path`: The path to the directory containing the raws.
/// * `sourced_dir`: The directory the root_path was found under.
///
/// Returns:
///
/// A JSON string containing the raws for the given directory.
pub fn parse_raw_module(root_path: &Path) -> String {
    parser::parse_raw_module_to_json_string(root_path)
}

/// Takes a path to a folder containing raws, and a path to a file, and parses the raws and saves
/// the result to the file as JSON
///
/// Arguments:
///
/// * `input_path`: The path to the raws folder.
/// * `output_file_path`: The path to the file to save the parsed raws to.
pub fn parse_game_raws_to_file(input_path: &str, out_filepath: &Path) {
    let json_string_vec = parse_game_raws(input_path);
    parser::util::write_json_string_array_to_file(json_string_vec, out_filepath);
}

/// Takes a path to a folder containing raws, and a path to a file, and parses the raws and saves
/// the result to the file as JSON
///
/// Arguments:
///
/// * `input_path`: The path to the raws folder.
/// * `output_file_path`: The path to the file to save the parsed raws to.
pub fn parse_all_raw_module_info_to_file(input_path: &str, out_filepath: &Path) {
    let json_string_vec = parse_all_raw_module_info(input_path);
    parser::util::write_json_string_array_to_file(json_string_vec, out_filepath);
}

#[cfg(feature = "tauri")]
#[derive(Clone, serde::Serialize)]
/// It's a struct to represent the progress of the current job. This is emitted back to the Tauri app using the `PROGRESS` event.
///
/// Properties:
///
/// * `percentage`: The percentage of completed steps out of total steps.
/// * `current_module`: The name of the module that is currently being processed.
struct ProgressPayload {
    percentage: f64,
    #[serde(rename = "currentModule")]
    current_module: String,
}

#[cfg(feature = "tauri")]
/// Parse a directory of raws, and return a JSON string of the parsed raws. While parsing, this will
/// emit tauri events to the supplied window. The event is titled `PROGRESS` and it uses the `ProgressPayload`
/// payload for the payload.
///
/// The payload supplies the current progress as a float and the name of the current folder being parsed.
///
/// Properties:
///
/// * `df_game_path`: The path to the Dwarf Fortress install directory
/// * `window`: A tauri::Window to emit `PROGRESS` events to.
///
/// Returns:
///
/// A (large) JSON string with details on all raws in the game path.
pub fn parse_game_raws_with_tauri_emit(df_game_path: &str, window: tauri::Window) -> Vec<String> {
    let mut all_json: Vec<String> = Vec::new();

    //1. "validate" folder is as expected
    let game_path = Path::new(df_game_path);
    // Guard against invalid path
    if !game_path.exists() {
        log::error!(
            "Provided game path for parsing doesn't exist!\n{}",
            df_game_path
        );
        return all_json;
    }
    if !game_path.is_dir() {
        log::error!("Game path needs to be a directory {}", df_game_path);
        return all_json;
    }

    // warn on no gamelog.txt
    if !game_path.join("gamelog.txt").exists() {
        log::warn!("Unable to find gamelog.txt in game directory. Is it valid?");
    }

    let data_path = game_path.join("data");
    let vanilla_path = data_path.join("vanilla");
    let installed_mods_path = data_path.join("installed_mods");
    let workshop_mods_path = game_path.join("mods");

    let vanilla_iter: Vec<DirEntry> = subdirectories(vanilla_path).unwrap_or(Vec::new());
    let installed_iter: Vec<DirEntry> = subdirectories(installed_mods_path).unwrap_or(Vec::new());
    let mods_iter: Vec<DirEntry> = subdirectories(workshop_mods_path).unwrap_or(Vec::new());

    // Calculate total number of modules we will parse:
    let total_mods = vanilla_iter.len() + installed_iter.len() + mods_iter.len();
    let mut current_mod: usize = 0;
    let mut pct = 0.0;

    for entry in vanilla_iter {
        current_mod = current_mod + 1;
        pct = current_mod as f64 / total_mods as f64;
        match window.emit(
            "PROGRESS",
            ProgressPayload {
                percentage: pct,
                current_module: String::from(entry.file_name().to_str().unwrap_or("")),
            },
        ) {
            Err(e) => log::debug!("Tauri window emit error {:?}", e),
            _ => (),
        };

        all_json.push(parse_raw_module(entry.path()));
    }
    for entry in installed_iter {
        current_mod = current_mod + 1;
        pct = current_mod as f64 / total_mods as f64;
        match window.emit(
            "PROGRESS",
            ProgressPayload {
                percentage: pct,
                current_module: String::from(entry.file_name().to_str().unwrap_or("")),
            },
        ) {
            Err(e) => log::debug!("Tauri window emit error {:?}", e),
            _ => (),
        };

        all_json.push(parse_raw_module(entry.path()));
    }
    for entry in mods_iter {
        current_mod = current_mod + 1;
        pct = current_mod as f64 / total_mods as f64;
        match window.emit(
            "PROGRESS",
            ProgressPayload {
                percentage: pct,
                current_module: format!(
                    "workshop-mod:{}",
                    entry.file_name().to_str().unwrap_or("unknown")
                ),
            },
        ) {
            Err(e) => log::debug!("Tauri window emit error {:?}", e),
            _ => (),
        };

        all_json.push(parse_raw_module(entry.path()));
    }

    if pct < 1.0 {
        pct = 1.0;
        match window.emit(
            "PROGRESS",
            ProgressPayload {
                percentage: pct,
                current_module: String::new(),
            },
        ) {
            Err(e) => log::debug!("Tauri window emit error {:?}", e),
            _ => (),
        };
    }

    let non_empty_json: Vec<String> = all_json
        .into_iter()
        .filter(|s| !String::is_empty(s))
        .collect();

    non_empty_json
}

/// It takes a path to a directory, checks if it's a directory, checks if it has an info.txt file,
/// parses the info.txt file, and then serializes the parsed data into a JSON string
///
/// Arguments:
///
/// * `root_path`: The path to the directory containing the raws.
///
/// Returns:
///
/// A string.
pub fn parse_info_txt_from_raw_module(raw_module_directory: &Path) -> String {
    let dfraw_module_info = parser::parse_info_file_from_module_directory(&raw_module_directory);
    match dfraw_module_info.to_typed_json_string() {
        Ok(s) => {
            return s.to_string();
        }
        Err(e) => {
            log::error!("Failure to serialize parsed raw data\n{}", e);
        }
    }

    String::new()
}

/// It takes a path to a file, parses it, and returns a JSON string
///
/// Arguments:
///
/// * `raw_file`: The path to the raw file to parse.
///
/// Returns:
///
/// A JSON String
pub fn parse_single_raw_file(raw_file: &Path) -> String {
    parser::parse_single_raw_file_to_json_string(raw_file)
}

/// Takes the path to the DF game directory, parses the raw module info files and then
/// writes the JSON strings of those parsed modules to the out_filepath
///
/// Arguments:
///
/// * `df_game_path`: The path to the the DF game directory.
/// * `out_filepath`: The path to the file you want to write to.
pub fn parse_all_raw_module_info(df_game_path: &str) -> Vec<String> {
    //1. "validate" folder is as expected
    let game_path = Path::new(df_game_path);
    // Guard against invalid path
    if !game_path.exists() {
        log::error!(
            "Provided game path for parsing doesn't exist!\n{}",
            df_game_path
        );
        return Vec::new();
    }
    if !game_path.is_dir() {
        log::error!("Game path needs to be a directory {}", df_game_path);
        return Vec::new();
    }

    // warn on no gamelog.txt
    if !game_path.join("gamelog.txt").exists() {
        log::warn!("Unable to find gamelog.txt in game directory. Is it valid?");
    }

    // Set file paths for vanilla raw modules, workshop mods and installed mods
    let data_path = game_path.join("data");
    let vanilla_path = data_path.join("vanilla");
    let installed_mods_path = data_path.join("installed_mods");
    let workshop_mods_path = game_path.join("mods");

    let mut all_json: Vec<String> = Vec::new();

    all_json.push(parse_info_txt_in_module_location(&vanilla_path.as_path()));
    all_json.push(parse_info_txt_in_module_location(
        &installed_mods_path.as_path(),
    ));
    all_json.push(parse_info_txt_in_module_location(
        &workshop_mods_path.as_path(),
    ));

    all_json
}

/// It reads a single raw file, parses it, and writes the parsed JSON string to a file
///
/// Arguments:
///
/// * `raw_file`: The path to the raw file to read.
/// * `out_filepath`: The path to the file you want to write to.
pub fn read_single_raw_file_to_file(raw_file: &Path, out_filepath: &Path) {
    let parsed_json_string = parse_single_raw_file(raw_file);
    parser::util::write_json_string_to_file(parsed_json_string, out_filepath);
}

/// Parses a single raw module directory, and writes the parsed JSON string to a file
///
/// Arguments:
///
/// * `module_path`: The path to the raw file to read.
/// * `out_filepath`: The path to the file you want to write to.
pub fn parse_raw_module_to_file(module_path: &Path, out_filepath: &Path) {
    let parsed_json_string = parse_raw_module(module_path);
    parser::util::write_json_string_to_file(parsed_json_string, out_filepath);
}

/// Parses all the raw modules within a raw module location, and writes the parsed JSON string to a file
///
/// Arguments:
///
/// * `raw_module_location_path`: The path to the raw module directory.
/// * `out_filepath`: The path to the file you want to write to.
pub fn parse_raw_module_location_dir_to_file(raw_module_location_path: &Path, out_filepath: &Path) {
    let parsed_json_string = parse_module_location(raw_module_location_path);
    parser::util::write_json_string_to_file(parsed_json_string, out_filepath);
}
