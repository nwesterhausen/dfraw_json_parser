#[cfg(feature = "tauri")]
extern crate tauri;
#[cfg(feature = "tauri")]
use crate::{
    parse_raw_module,
    parser::util::{path_from_game_directory, subdirectories},
};
#[cfg(feature = "tauri")]
use walkdir::DirEntry;

use std::path::Path;
#[cfg(feature = "tauri")]
#[derive(Clone, serde::Serialize)]
/// It's a struct to represent the progress of the current job. This is emitted back to the Tauri app using the `PROGRESS` event.
///
/// Properties:
///
/// * `percentage`: The percentage of completed steps out of total steps.
/// * `current_module`: The name of the module that is currently being processed.
pub struct ProgressPayload {
    percentage: f64,
    #[serde(rename = "currentModule")]
    current_module: String,
    #[serde(rename = "currentTask")]
    current_task: String,
}

#[cfg(feature = "tauri")]
#[allow(clippy::cast_precision_loss)]
/// Parse a directory of raws, and return a JSON string of the parsed raws. While parsing, this will
/// emit tauri events to the supplied window. The event is titled `PROGRESS` and it uses the `ProgressPayload`
/// payload for the payload.
///
/// The payload supplies the current progress as a float and the name of the current folder being parsed.
///
/// Properties:
///
/// * `df_game_path`: The path to the Dwarf Fortress install directory
/// * `window`: A `tauri::Window` to emit `PROGRESS` events to.
///
/// Returns:
///
/// A (large) JSON string with details on all raws in the game path.
pub fn parse_game_raws_with_tauri_emit<P: AsRef<Path>>(
    df_game_path: &P,
    window: &tauri::Window,
) -> String {
    // Validate game path
    let game_path = match path_from_game_directory(df_game_path) {
        Ok(path_buf) => path_buf,
        Err(e) => {
            log::error!("{}", e);
            return String::new();
        }
    };

    // Build location directory paths
    let data_path = game_path.join("data");
    let vanilla_path = data_path.join("vanilla");
    let installed_mods_path = data_path.join("installed_mods");
    let workshop_mods_path = game_path.join("mods");

    // let vanilla_iter: Vec<DirEntry> = subdirectories(vanilla_path).unwrap_or_default();
    // let installed_iter: Vec<DirEntry> = subdirectories(installed_mods_path).unwrap_or_default();
    // let mods_iter: Vec<DirEntry> = subdirectories(workshop_mods_path).unwrap_or_default();

    // Calculate total number of modules we will parse:
    // let total_mods = vanilla_iter.len() + installed_iter.len() + mods_iter.len();
    // let mut current_mod: usize = 0;
    // let mut pct = 0.0;

    let all_json = vec![
        parse_location_with_tauri_emit(&vanilla_path, window),
        parse_location_with_tauri_emit(&installed_mods_path, window),
        parse_location_with_tauri_emit(&workshop_mods_path, window),
    ];

    let non_empty_json: Vec<String> = all_json
        .into_iter()
        .filter(|s| !String::is_empty(s))
        .collect();

    format!("[{}]", non_empty_json.join(","))
}

#[cfg(feature = "tauri")]
#[allow(clippy::cast_precision_loss)]
/// It takes a path to a directory containing raw modules, parses them, and returns a JSON string
/// containing all the parsed modules. While parsing, emits events to the provided tauri window
/// to convey parsing status.
///
/// Arguments:
///
/// * `raw_module_location`: The path to the directory containing the raw modules.
/// * `window`: The active tauri window to receive events.
///
/// Returns:
///
/// A JSON string of all the mods in the location.
pub fn parse_location_with_tauri_emit<P: AsRef<Path>>(
    raw_module_location: &P,
    window: &tauri::Window,
) -> String {
    let raw_module_location_path = raw_module_location.as_ref();
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
    let module_location = crate::parser::raws::RawModuleLocation::from_sourced_directory(
        raw_module_location_path
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default(),
    );

    //3. Get list of all subdirectories
    let raw_module_iter: Vec<DirEntry> =
        subdirectories(std::path::PathBuf::from(raw_module_location_path)).unwrap_or_default();

    log::info!(
        "{num} raw modules located in {location:?}",
        num = raw_module_iter.len(),
        location = module_location
    );

    // Calculate total number of modules we will parse:
    let total_mods = raw_module_iter.len();
    let mut current_mod: usize = 0;
    let mut pct = 0.0;
    let current_task = format!("Parsing raws in {:?}", module_location);

    let mut all_json: Vec<String> = Vec::new();
    //4. Loop over all raw modules in the raw module directory
    for raw_module_directory in raw_module_iter {
        current_mod += 1;
        pct = current_mod as f64 / total_mods as f64;
        if let Err(e) = window.emit(
            "PROGRESS",
            ProgressPayload {
                percentage: pct,
                current_module: format!(
                    "module_location:{}",
                    raw_module_directory
                        .file_name()
                        .to_str()
                        .unwrap_or("unknown")
                ),
                current_task: current_task.clone(),
            },
        ) {
            log::debug!("Tauri window emit error {:?}", e);
        };
        //2. Parse raws and dump JSON into array
        all_json.push(parse_raw_module(&raw_module_directory.path()));
    }

    // Finally send 100% message
    if pct < 1.0 {
        pct = 1.0;
        if let Err(e) = window.emit(
            "PROGRESS",
            ProgressPayload {
                percentage: pct,
                current_module: String::new(),
                current_task: String::from("None"),
            },
        ) {
            log::debug!("Tauri window emit error {:?}", e);
        };
    }

    let non_empty_json: Vec<String> = all_json
        .into_iter()
        .filter(|s| !String::is_empty(s))
        .collect();

    format!("[{}]", non_empty_json.join(","))
}
