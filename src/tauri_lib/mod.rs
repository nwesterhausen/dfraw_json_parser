#[cfg(feature = "tauri")]
extern crate tauri;
#[cfg(feature = "tauri")]
use crate::{
    parse_raw_module,
    parser::util::{path_from_game_directory, subdirectories},
};
#[cfg(feature = "tauri")]
use walkdir::DirEntry;

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
    // Validate game path
    let game_path = match path_from_game_directory(df_game_path) {
        Ok(path_buf) => path_buf,
        Err(e) => {
            log::error!("{}", e);
            return Vec::new();
        }
    };

    // Build location directory paths
    let data_path = game_path.join("data");
    let vanilla_path = data_path.join("vanilla");
    let installed_mods_path = data_path.join("installed_mods");
    let workshop_mods_path = game_path.join("mods");

    let vanilla_iter: Vec<DirEntry> = subdirectories(vanilla_path).unwrap_or_default();
    let installed_iter: Vec<DirEntry> = subdirectories(installed_mods_path).unwrap_or_default();
    let mods_iter: Vec<DirEntry> = subdirectories(workshop_mods_path).unwrap_or_default();

    // Calculate total number of modules we will parse:
    let total_mods = vanilla_iter.len() + installed_iter.len() + mods_iter.len();
    let mut current_mod: usize = 0;
    let mut pct = 0.0;

    let mut all_json: Vec<String> = Vec::new();

    for entry in vanilla_iter {
        current_mod += 1;
        pct = current_mod as f64 / total_mods as f64;
        if let Err(e) = window.emit(
            "PROGRESS",
            ProgressPayload {
                percentage: pct,
                current_module: String::from(entry.file_name().to_str().unwrap_or("")),
                current_task: String::from("Parse vanilla raw modules"),
            },
        ) {
            log::debug!("Tauri window emit error {:?}", e)
        };

        all_json.push(parse_raw_module(entry.path()));
    }
    for entry in installed_iter {
        current_mod += 1;
        pct = current_mod as f64 / total_mods as f64;
        if let Err(e) = window.emit(
            "PROGRESS",
            ProgressPayload {
                percentage: pct,
                current_module: String::from(entry.file_name().to_str().unwrap_or("")),
                current_task: String::from("Parse installed_mods raw modules"),
            },
        ) {
            log::debug!("Tauri window emit error {:?}", e)
        };

        all_json.push(parse_raw_module(entry.path()));
    }
    for entry in mods_iter {
        current_mod += 1;
        pct = current_mod as f64 / total_mods as f64;
        if let Err(e) = window.emit(
            "PROGRESS",
            ProgressPayload {
                percentage: pct,
                current_module: format!(
                    "workshop-mod:{}",
                    entry.file_name().to_str().unwrap_or("unknown")
                ),
                current_task: String::from("Parse workshop raw modules"),
            },
        ) {
            log::debug!("Tauri window emit error {:?}", e)
        };

        all_json.push(parse_raw_module(entry.path()));
    }

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
            log::debug!("Tauri window emit error {:?}", e)
        };
    }

    let non_empty_json: Vec<String> = all_json
        .into_iter()
        .filter(|s| !String::is_empty(s))
        .collect();

    non_empty_json
}
