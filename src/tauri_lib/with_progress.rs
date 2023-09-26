#[cfg(feature = "tauri")]
extern crate tauri;
#[cfg(feature = "tauri")]
use crate::parser;
use crate::parser::raws::RawObject;
#[cfg(feature = "tauri")]
use crate::util;
#[cfg(feature = "tauri")]
use walkdir::DirEntry;

#[cfg(feature = "tauri")]
use super::structs::ProgressHelper;
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
pub fn parse_game_raws<P: AsRef<Path>>(
    df_game_path: &P,
    progress_helper: &mut ProgressHelper,
) -> String {
    // Validate game path
    let game_path = match util::path_from_game_directory(df_game_path) {
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

    let all_json = vec![
        parse_location(&vanilla_path, progress_helper),
        parse_location(&installed_mods_path, progress_helper),
        parse_location(&workshop_mods_path, progress_helper),
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
pub fn parse_location<P: AsRef<Path>>(
    raw_module_location: &P,
    progress_helper: &mut ProgressHelper,
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
    let module_location = crate::parser::raw_locations::RawModuleLocation::from_sourced_directory(
        raw_module_location_path
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default(),
    );

    //3. Get list of all subdirectories
    let raw_module_iter: Vec<DirEntry> =
        util::subdirectories(std::path::PathBuf::from(raw_module_location_path))
            .unwrap_or_default();

    log::info!(
        "{num} raw modules located in {location:?}",
        num = raw_module_iter.len(),
        location = module_location
    );
    progress_helper.update_current_location(format!("{module_location:?}").as_str());

    // Calculate total number of modules we will parse:
    progress_helper.add_steps(raw_module_iter.len());
    progress_helper.update_current_task(format!("Parsing raws in {module_location:?}").as_str());

    let mut all_json: Vec<String> = Vec::new();
    //4. Loop over all raw modules in the raw module directory
    for raw_module_directory in raw_module_iter {
        //2. Parse raws and dump JSON into array
        all_json.push(parse_module(&raw_module_directory.path(), progress_helper));
    }

    let non_empty_json: Vec<String> = all_json
        .into_iter()
        .filter(|s| !String::is_empty(s))
        .collect();

    format!("[{}]", non_empty_json.join(","))
}

pub fn parse_module<P: AsRef<Path>>(
    raw_module_directory: &P,
    progress_helper: &mut ProgressHelper,
) -> String {
    //1. Get information from the info.txt file
    if !raw_module_directory.as_ref().exists() {
        log::error!(
            "Provided directory to parse raws does not exist: {:?}",
            raw_module_directory.as_ref().to_string_lossy()
        );
        return String::new();
    }
    if !raw_module_directory.as_ref().is_dir() {
        log::error!(
            "Provided 'directory' to parse is not actually a directory! {:?}",
            raw_module_directory.as_ref().to_string_lossy()
        );
        return String::new();
    }

    // Check for info.txt
    let info_txt_path = raw_module_directory.as_ref().join("info.txt");
    if !info_txt_path.exists() {
        let dir_name = raw_module_directory
            .as_ref()
            .file_name()
            .unwrap_or_default();
        let dir_name_str = dir_name.to_str().unwrap_or("");

        if !(dir_name_str.eq("mod_upload")
            || dir_name_str.eq("examples and notes")
            || dir_name_str.eq("interaction examples"))
        {
            log::error!(
                "No info.txt as expected in {:?}. Is this DF 50.xx?",
                raw_module_directory
                    .as_ref()
                    .file_name()
                    .unwrap_or_default()
            );
        }

        return String::new();
    }

    // Parse info.txt to get raw module information
    let dfraw_module_info = parser::mod_info_file::ModuleInfoFile::parse(&info_txt_path);
    log::info!(
        "Parsing raws for {} v{}",
        dfraw_module_info.get_identifier(),
        dfraw_module_info.get_version()
    );
    progress_helper.update_current_module(
        format!(
            "{} v{}",
            dfraw_module_info.get_identifier(),
            dfraw_module_info.get_version()
        )
        .as_str(),
    );
    //2. Parse raws in the 'object' subdirectory
    let objects_path = raw_module_directory.as_ref().join("objects");
    if !objects_path.exists() {
        log::debug!("No objects subdirectory, no raws to parse.");
    }
    if !objects_path.is_dir() {
        log::error!("Objects subdirectory is not valid subdirectory! Unable to parse raws.");
    }
    //2. Parse raws in the 'object' subdirectory
    let graphics_path = raw_module_directory.as_ref().join("graphics");
    if !graphics_path.exists() {
        log::debug!("No graphics subdirectory, no raws to parse.");
    }
    if !graphics_path.is_dir() {
        log::error!("Graphics subdirectory is not valid subdirectory! Unable to parse raws.");
    }

    if !objects_path.exists() && !graphics_path.exists() {
        //exit because nothing to parse
        return String::new();
    }

    // Setup empty result vector
    // let mut serializable_raws: Vec<Box<dyn TypedJsonSerializable>> = Vec::new();
    let mut serializable_raws: Vec<Box<dyn RawObject>> = Vec::new();

    // Read all the files in the directory, selectively parse the .txt files
    for entry in walkdir::WalkDir::new(objects_path)
        .into_iter()
        .filter_map(std::result::Result::ok)
    {
        let f_name = entry.file_name().to_string_lossy();

        if f_name.ends_with(".txt") {
            progress_helper.add_steps(1);
            progress_helper.send_update(&f_name);
            let entry_path = entry.path();
            serializable_raws.extend(parser::parse_raws_from_single_file(&entry_path, false));
        }
    }
    // Read all the files in the directory, selectively parse the .txt files
    for entry in walkdir::WalkDir::new(graphics_path)
        .into_iter()
        .filter_map(std::result::Result::ok)
    {
        let f_name = entry.file_name().to_string_lossy();

        if f_name.ends_with(".txt") {
            progress_helper.add_steps(1);
            progress_helper.send_update(&f_name);
            let entry_path = entry.path();
            serializable_raws.extend(parser::parse_raws_from_single_file(&entry_path, false));
        }
    }

    serde_json::to_string(&serializable_raws).unwrap_or_default()
}
