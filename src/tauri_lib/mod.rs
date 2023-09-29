#[cfg(feature = "tauri")]
extern crate tauri;

#[cfg(feature = "tauri")]
mod structs;
#[cfg(feature = "tauri")]
mod with_progress;

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
pub fn parse(options: &crate::options::ParserOptions, window: tauri::Window) -> String {
    // setup progress helper
    let mut progress_helper = structs::ProgressHelper::with_tauri_window(window);
    progress_helper.update_current_task("Parsing all raws in dwarf fortress directory.");

    let result = with_progress::parse(options, &mut progress_helper);
    progress_helper.send_final("Parsing completed.");

    crate::util::raws_to_string(result)
}

#[cfg(feature = "tauri")]
/// The function `parse_to_json_vec` takes in options and a window, sets up a progress helper, parses
/// raws in a Dwarf Fortress directory, and returns a vector of strings.
///
/// Arguments:
///
/// * `options`: A reference to a `ParserOptions` struct from the `crate::options` module.
/// * `window`: The `window` parameter is of type `tauri::Window` and represents the Tauri window
/// object. It is used to interact with the Tauri window and perform operations such as displaying
/// progress updates.
///
/// Returns:
///
/// a vector of JSON strings.
pub fn parse_to_json_vec(
    options: &crate::options::ParserOptions,
    window: tauri::Window,
) -> Vec<String> {
    // setup progress helper
    let mut progress_helper = structs::ProgressHelper::with_tauri_window(window);
    progress_helper.update_current_task("Parsing all raws in dwarf fortress directory.");

    let result = with_progress::parse_to_json_vec(options, &mut progress_helper);
    progress_helper.send_final("Parsing completed.");

    result
}
