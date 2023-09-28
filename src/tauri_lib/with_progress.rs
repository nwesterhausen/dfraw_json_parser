#[cfg(feature = "tauri")]
extern crate tauri;
#[cfg(feature = "tauri")]
use super::structs::ProgressHelper;
#[cfg(feature = "tauri")]
use crate::parser;
#[cfg(feature = "tauri")]
use crate::parser::raws::RawObject;
#[cfg(feature = "tauri")]
use crate::util;
#[cfg(feature = "tauri")]
use std::path::Path;
#[cfg(feature = "tauri")]
use walkdir::DirEntry;

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
    progress_helper: &mut ProgressHelper,
) -> String {
    // Guard against invalid path
    if !crate::util::is_valid_path(&options.target_path, &options.job) {
        log::error!(
            "Returning early for bad path. Provided options:\n{:#?}",
            options
        );
        return String::from("[]");
    }
    let target_path = Path::new(&options.target_path);
    let mut results: Vec<Box<dyn RawObject>> = Vec::new();

    match options.job {
        crate::options::ParsingJob::All => {
            // Set file paths for each location
            let data_path = target_path.join("data");
            let vanilla_path = data_path.join("vanilla");
            let installed_mods_path = data_path.join("installed_mods");
            let workshop_mods_path = target_path.join("mods");

            // Parse each location
            results.extend(parse_location(&vanilla_path, options, progress_helper));
            results.extend(parse_location(
                &installed_mods_path,
                options,
                progress_helper,
            ));
            results.extend(parse_location(
                &workshop_mods_path,
                options,
                progress_helper,
            ));
        }
        crate::options::ParsingJob::SingleLocation => {
            // Set the file path for the chosen location
            let location_path = if let Some(location) = options.locations_to_parse.first() {
                match location {
                    parser::raw_locations::RawModuleLocation::Vanilla => {
                        target_path.join("data").join("vanilla")
                    }
                    parser::raw_locations::RawModuleLocation::InstalledMods => {
                        target_path.join("data").join("installed_mods")
                    }
                    parser::raw_locations::RawModuleLocation::Mods => target_path.join("mods"),
                    parser::raw_locations::RawModuleLocation::Unknown => {
                        log::error!(
                            "Unknown location provided to parse! Provided options:\n{:#?}",
                            options
                        );
                        return String::from("[]");
                    }
                }
            } else {
                log::error!(
                    "No location provided to parse! Provided options:\n{:#?}",
                    options
                );
                return String::from("[]");
            };

            // Parse the location
            results.extend(parse_location(&location_path, options, progress_helper));
        }
        crate::options::ParsingJob::SingleModule => {
            // The provided path should be a module directory

            // Check for info.txt
            let info_txt_path = target_path.join("info.txt");
            if !info_txt_path.exists() {
                let dir_name = target_path.file_name().unwrap_or_default();
                let dir_name_str = dir_name.to_str().unwrap_or("");

                if !(dir_name_str.eq("mod_upload")
                    || dir_name_str.eq("examples and notes")
                    || dir_name_str.eq("interaction examples"))
                {
                    log::error!(
                        "No info.txt as expected in {:?}. Is this DF 50.xx? Provided options:\n{:#?}",
                        target_path.file_name().unwrap_or_default(),
                        options
                    );
                }

                return String::from("[]");
            }

            results.extend(parse_module(&target_path, options, progress_helper));
        }
        crate::options::ParsingJob::SingleRaw => {
            // The provided path should be a raw file directly
            results.extend(parser::parse_raws_from_single_file(&target_path, options));
        }
        crate::options::ParsingJob::SingleModuleInfoFile => {
            // The provided path should be the info.txt file for a module
            log::warn!(
                "Unable to parse info.txt file in this dispatch. Provided options:\n{:#?}",
                options
            );
            return String::from("[]");
        }
    }

    // Convert the results to a JSON string
    raws_to_string(results)
}

#[cfg(feature = "tauri")]
/// Convert the `Vec<Box<dyn RawObject>>` into a JSON string.
fn raws_to_string(raws: Vec<Box<dyn RawObject>>) -> String {
    // It should be an array, so start with '[' character,
    // then add each raw object, separated by a comma.
    // Finally add the closing ']' character.
    // (The last item cannot have a comma before ']')
    let mut json = String::from('[');
    for raw in raws {
        json.push_str(serde_json::to_string(&raw).unwrap().as_str());
        json.push(',');
    }
    json.pop(); // remove trailing comma
    json.push(']');
    json
}

#[cfg(feature = "tauri")]
/// Parses the raws in the provided location path, and returns a vector of boxed dynamic raw objects.
///
/// This is meant to be a private function, because the main entry point should be `parse`.
///
/// # Arguments
///
/// * `location_path` - A reference to the path to parse.
/// * `options` - A reference to a `ParserOptions` struct that contains the parsing options.
/// * `progress` - A reference to a `ProgressHelper` struct that contains the progress information.
///
/// # Returns
///
/// A vector of boxed dynamic raw objects.
fn parse_location<P: AsRef<Path>>(
    location_path: &P,
    options: &crate::options::ParserOptions,
    progress_helper: &mut ProgressHelper,
) -> Vec<Box<dyn RawObject>> {
    let mut results: Vec<Box<dyn RawObject>> = Vec::new();
    let location_path: std::path::PathBuf = location_path.as_ref().to_path_buf();
    // Get a list of all subdirectories in the location
    let raw_modules_in_location: Vec<DirEntry> =
        util::subdirectories(location_path).unwrap_or_default();
    let module_location = options
        .locations_to_parse
        .first()
        .unwrap_or(&crate::parser::raw_locations::RawModuleLocation::Unknown);
    log::info!(
        "Found {} raw modules in {:?}",
        raw_modules_in_location.len(),
        module_location,
    );

    progress_helper.update_current_location(format!("{module_location:?}").as_str());

    // Calculate total number of modules we will parse:
    progress_helper.add_steps(raw_modules_in_location.len());
    progress_helper.update_current_task(format!("Parsing raws in {module_location:?}").as_str());

    // Loop over each module and parse it
    for raw_module in raw_modules_in_location {
        let module = parse_module(&raw_module.path(), options, progress_helper);
        results.extend(module);
    }

    results
}

#[cfg(feature = "tauri")]
fn parse_module<P: AsRef<Path>>(
    module_path: &P,
    options: &crate::options::ParserOptions,
    progress_helper: &mut ProgressHelper,
) -> Vec<Box<dyn RawObject>> {
    // Get information from the module info file
    let module_info_file_path = module_path.as_ref().join("info.txt");
    let module_info_file = crate::parse_module_info_file_direct(&module_info_file_path);

    log::info!(
        "Parsing raws for {} v{}",
        module_info_file.get_identifier(),
        module_info_file.get_version(),
    );
    progress_helper.update_current_module(
        format!(
            "{} v{}",
            module_info_file.get_identifier(),
            module_info_file.get_version()
        )
        .as_str(),
    );

    // Get a list of all raw files in the module
    let objects_path = module_path.as_ref().join("objects");
    let graphics_path = module_path.as_ref().join("graphics");

    let mut parse_objects = true;
    let mut parse_graphics = false;

    if !objects_path.exists() {
        log::warn!(
            "No objects directory found in {:?}",
            module_path.as_ref().file_name().unwrap_or_default(),
        );
        parse_objects = false;
    }

    if parse_objects && !objects_path.is_dir() {
        log::warn!(
            "Objects directory in {:?} is not a directory",
            module_path.as_ref().file_name().unwrap_or_default(),
        );
        parse_objects = false;
    }

    if !graphics_path.exists() {
        log::warn!(
            "No graphics directory found in {:?}",
            module_path.as_ref().file_name().unwrap_or_default(),
        );
        parse_graphics = false;
    }

    if parse_graphics && !graphics_path.is_dir() {
        log::warn!(
            "Graphics directory in {:?} is not a directory",
            module_path.as_ref().file_name().unwrap_or_default(),
        );
        parse_graphics = false;
    }

    // Exit early if nothing to parse
    if !parse_graphics && !parse_objects {
        return Vec::new();
    }

    let mut results = Vec::new();

    // Parse the objects
    if parse_objects {
        for entry in walkdir::WalkDir::new(objects_path)
            .into_iter()
            .filter_map(std::result::Result::ok)
        {
            if entry.file_type().is_file() {
                let file_path = entry.path();
                let file_name = file_path.file_name().unwrap_or_default();
                let file_name_str = file_name.to_str().unwrap_or_default();

                if std::path::Path::new(file_name_str)
                    .extension()
                    .map_or(false, |ext| ext.eq_ignore_ascii_case("txt"))
                {
                    progress_helper.add_steps(1);
                    progress_helper.send_update(file_name_str);
                    results.extend(parser::parse_raws_from_single_file(&file_path, options));
                }
            }
        }
    }

    // Parse the graphics
    // NOT IMPLEMENTED YET

    results
}
