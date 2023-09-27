#[cfg(feature = "tauri")]
extern crate tauri;

#[cfg(feature = "tauri")]
mod structs;
#[cfg(feature = "tauri")]
mod with_progress;

use std::path::Path;

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
/// * `window`: A `tauri::Window` to emit `PROGRESS` events to.
///
/// Returns:
///
/// A (large) JSON string with details on all raws in the game path.
pub fn parse_game_raws_with_tauri_emit<P: AsRef<Path>>(
    df_game_path: &P,
    window: tauri::Window,
    options: Option<&crate::options::ParserOptions>,
) -> String {
    // setup progress helper
    let mut progress_helper = structs::ProgressHelper::with_tauri_window(window);
    progress_helper.update_current_task("Parsing all raws in dwarf fortress directory.");

    let result = with_progress::parse_game_raws(df_game_path, &mut progress_helper, options);
    progress_helper.send_final("Parsing completed.");

    result
}

#[cfg(feature = "tauri")]
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
    window: tauri::Window,
    options: Option<&crate::options::ParserOptions>,
) -> String {
    // setup progress helper
    let mut progress_helper = structs::ProgressHelper::with_tauri_window(window);
    progress_helper.update_current_task("Parsing all raws in a single location.");

    let result = with_progress::parse_location(raw_module_location, &mut progress_helper, options);
    progress_helper.send_final("Parsing completed.");

    result
}
