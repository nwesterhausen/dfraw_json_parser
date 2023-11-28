#[cfg(feature = "tauri")]
extern crate tauri;

#[cfg(feature = "tauri")]
mod structs;
#[cfg(feature = "tauri")]
mod with_progress;

#[cfg(feature = "tauri")]
pub use structs::ProgressPayload;

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
pub fn parse(
    options: &crate::options::ParserOptions,
    window: tauri::Window,
) -> Result<crate::ParseResult, crate::ParserError> {
    // setup progress helper
    let mut progress_helper = structs::ProgressHelper::with_tauri_window(window);
    progress_helper.update_current_task("Parsing raws.");

    let result = with_progress::parse(options, &mut progress_helper)?;

    progress_helper.send_final("Parsing completed.");

    Ok(result)
}
