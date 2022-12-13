/*!
dfraw_json_parser provides a way to turn raw files from Dwarf Fortress into JSON.
It functions using a regular expression to break apart the tokens and then checks the
key in the token against a long list of ones it knows about. Any matches are utilized to
build its own representation of the raw before a final step where it dumps it to JSON.

Currently the only raws that are parsed are Creature raws.

Currently the JSON is returned as a string
*/

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

#[cfg(feature = "tauri")]
extern crate tauri;

mod parser;

//Todo: add an option to apply any COPY_TAG_FROM and APPLY_CREATURE_VARIATION tags
//Todo: add an option to specify what kinds of raws to parse
//Todo: add a parameter for JSON_FILE name (currently hard-coded to out.json)

/// It takes a path to a DF game directory, and returns a JSON array of all the raws in that directory
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

    let data_path = game_path.join("data");
    let vanilla_path = data_path.join("vanilla");
    let installed_mods_path = data_path.join("installed_mods");
    let workshop_mods_path = game_path.join("mods");

    let vanilla_iter: Vec<DirEntry> = subdirectories(vanilla_path).unwrap_or(Vec::new());
    let installed_iter: Vec<DirEntry> = subdirectories(installed_mods_path).unwrap_or(Vec::new());
    let mods_iter: Vec<DirEntry> = subdirectories(workshop_mods_path).unwrap_or(Vec::new());

    // Loop through each dfraw_directory and parse it
    for entry in vanilla_iter {
        all_json.push(parse_dfraw_dir(entry.path()));
    }

    for entry in installed_iter {
        all_json.push(parse_dfraw_dir(entry.path()));
    }

    for entry in mods_iter {
        all_json.push(parse_dfraw_dir(entry.path()));
    }

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
///
/// Returns:
///
/// A JSON string containing the raws for the given directory.
pub fn parse_dfraw_dir(root_path: &Path) -> String {
    //1. Get information from the info.txt file
    if !root_path.exists() {
        log::error!(
            "Provided directory to parse raws does not exist: {:?}",
            root_path
        );
        return String::new();
    }
    if !root_path.is_dir() {
        log::error!(
            "Provided 'directory' to parse is not actually a directory! {:?}",
            root_path
        );
        return String::new();
    }

    // Check for info.txt
    let info_txt_path = root_path.join("info.txt");
    if !info_txt_path.exists() {
        let Some(dir_name) = root_path.file_name() else {
            log::error!("Error reading module dir {:?}", root_path);
            return String::new();
        };
        let dir_name_str = dir_name.to_str().unwrap_or("");

        if !(dir_name_str.eq("mod_upload")
            || dir_name_str.eq("examples and notes")
            || dir_name_str.eq("interaction examples"))
        {
            log::error!(
                "No info.txt as expected in {:?}. Is this DF 50.xx?",
                root_path.file_name().unwrap_or_default()
            );
        }

        return String::new();
    }

    let dfraw_module_info = parser::parse_info_file(&info_txt_path);
    log::info!(
        "Parsing raws for {} v{}",
        dfraw_module_info.get_identifier(),
        dfraw_module_info.displayed_version
    );

    //2. Parse raws in the 'object' subdirectory
    let objects_path = root_path.join("objects");
    if !objects_path.exists() {
        log::warn!("No objects subdirectory, no raws to parse.");
        return String::new();
    }
    if !objects_path.is_dir() {
        log::error!("Objects subdirectory is not valid subdirectory! Unable to parse raws.");
        return String::new();
    }

    //perform "normal" raws parsing on contents.. but we should inject our information from info.txt
    let json = parser::parse_directory_to_json_string(
        &objects_path,
        dfraw_module_info.get_identifier().as_str(),
        dfraw_module_info.displayed_version.as_str(),
    );

    //3. Return the object array for this dfraw dir
    json
}

/// It takes a path to a folder containing raws, and a path to a file, and it parses the raws and saves
/// the result to the file as JSON
///
/// Arguments:
///
/// * `input_path`: The path to the raws folder.
/// * `output_file_path`: The path to the file to save the parsed raws to.
pub fn parse_game_raws_to_file(input_path: &str, out_filepath: &Path) {
    let json_string_vec = parse_game_raws(input_path);
    log::info!("Saving json to to {:?}", out_filepath);

    let out_file = match File::create(&out_filepath) {
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

    let mut json_string_iter = json_string_vec.into_iter().peekable();
    while let Some(json_string) = json_string_iter.next() {
        match write!(stream, "{}", json_string) {
            Ok(_x) => (),
            Err(e) => {
                log::error!("{}\n{:?}", write_error, e);
                return;
            }
        };

        if json_string_iter.peek().is_some() {
            match write!(stream, ",") {
                Ok(_x) => (),
                Err(e) => {
                    log::error!("{}\n{:?}", write_error, e);
                    return;
                }
            };
        }

        match stream.flush() {
            Ok(_x) => (),
            Err(e) => {
                log::error!("{}\n{:?}", write_error, e);
                return;
            }
        };
    }

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

/// Get a vec of subdirectories for a given directory
///
/// Using the WalkDir crate:
/// 1. create a new WalkDir for `directory`
/// 2. limit to immediate contents (max_depth and min_depth at 1)
/// 3. as an iterator..
///     4. filter_map into only non-error results
///     5. filter into only directories
/// 4. collect as a vec
///
/// Arguments:
///
/// * `directory`: The directory to search in
///
/// Returns:
///
/// A vector of all subdirectories as walkdir::DirEntry
fn subdirectories(directory: std::path::PathBuf) -> Option<Vec<walkdir::DirEntry>> {
    if !(directory.exists() && directory.is_dir()) {
        return None;
    }
    Some(
        WalkDir::new(directory)
            .max_depth(1)
            .min_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_dir())
            .collect(),
    )
}

/// It takes a path to a directory of raws, and a path to a file, and it parses the raws into JSON and saves as the file.
///
/// Arguments:
///
/// * `input_path`: The path to the raws directory.
/// * `output_file_path`: The path to the file you want to write the JSON to.
pub fn parse_game_raws_to_file_out(input_path: &str, output_file_path: &Path) {
    parser::parse_directory_to_json_file(output_file_path, input_path, input_path, output_file_path)
}

#[cfg(feature = "tauri")]
#[derive(Clone, serde::Serialize)]
struct Payload {
    pct: f64,
}

#[cfg(feature = "tauri")]
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
    let mut pct;

    for entry in vanilla_iter {
        current_mod = current_mod + 1;
        pct = current_mod as f64 / total_mods as f64;
        match window.emit("PROGRESS", Payload { pct }) {
            Err(e) => log::debug!("Tauri window emit error {:?}", e),
            _ => (),
        };

        all_json.push(parse_dfraw_dir(entry.path()));
    }
    for entry in installed_iter {
        current_mod = current_mod + 1;
        pct = current_mod as f64 / total_mods as f64;
        match window.emit("PROGRESS", Payload { pct }) {
            Err(e) => log::debug!("Tauri window emit error {:?}", e),
            _ => (),
        };

        all_json.push(parse_dfraw_dir(entry.path()));
    }
    for entry in mods_iter {
        current_mod = current_mod + 1;
        pct = current_mod as f64 / total_mods as f64;
        match window.emit("PROGRESS", Payload { pct }) {
            Err(e) => log::debug!("Tauri window emit error {:?}", e),
            _ => (),
        };

        all_json.push(parse_dfraw_dir(entry.path()));
    }

    if pct < 1.0 {
        pct = 1.0;
        match window.emit("PROGRESS", Payload { pct }) {
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
