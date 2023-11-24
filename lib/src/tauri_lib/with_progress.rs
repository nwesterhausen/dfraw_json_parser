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
#[allow(clippy::too_many_lines)]
pub fn parse(
    options: &crate::options::ParserOptions,
    progress_helper: &mut ProgressHelper,
) -> Vec<Box<dyn RawObject>> {
    // Guard against invalid path

    use tracing::error;
    let Some(options) = crate::util::validate_options(options) else {
        error!("Options failed to validate\n{:#?}", options);
        return Vec::new();
    };

    let mut results: Vec<Box<dyn RawObject>> = Vec::new();

    // No job is specified, instead it is inferred from the options
    // First we can check if any locations are specified
    if !options.locations_to_parse.is_empty() {
        // Parse all locations that are specified.
        let target_path = Path::new(&options.dwarf_fortress_directory);

        // Build paths for each location
        let data_path = target_path.join("data");
        let vanilla_path = data_path.join("vanilla");
        let installed_mods_path = data_path.join("installed_mods");
        let workshop_mods_path = target_path.join("mods");

        // Parse each location
        if options
            .locations_to_parse
            .contains(&crate::parser::raw_locations::RawModuleLocation::Vanilla)
        {
            results.extend(parse_location(&vanilla_path, &options, progress_helper));
        }
        if options
            .locations_to_parse
            .contains(&crate::parser::raw_locations::RawModuleLocation::InstalledMods)
        {
            results.extend(parse_location(
                &installed_mods_path,
                &options,
                progress_helper,
            ));
        }
        if options
            .locations_to_parse
            .contains(&crate::parser::raw_locations::RawModuleLocation::Mods)
        {
            results.extend(parse_location(
                &workshop_mods_path,
                &options,
                progress_helper,
            ));
        }
    }

    // Next we can check if any raw modules are specified
    if !options.raw_modules_to_parse.is_empty() {
        // Parse all raw modules that are specified.
        for raw_module in &options.raw_modules_to_parse {
            let target_path = Path::new(&raw_module);

            // Check for info.txt
            let info_txt_path = target_path.join("info.txt");
            if !info_txt_path.exists() {
                let dir_name = target_path.file_name().unwrap_or_default();
                let dir_name_str = dir_name.to_str().unwrap_or("");

                if !(dir_name_str.eq("mod_upload")
                    || dir_name_str.eq("examples and notes")
                    || dir_name_str.eq("interaction examples"))
                {
                    error!(
                        "No info.txt as expected in {:?}. Is this DF 50.xx? Provided options:\n{:#?}",
                        target_path.file_name().unwrap_or_default(),
                        options
                    );
                }

                return Vec::new();
            }

            results.extend(parse_module(&target_path, &options, progress_helper));
        }
    }

    // Next we can check if any raw files are specified
    if !options.raw_files_to_parse.is_empty() {
        // Parse all raw files that are specified.
        for raw_file in &options.raw_files_to_parse {
            let target_path = Path::new(&raw_file);

            results.extend(parser::parse_raws_from_single_file(&target_path, &options));
        }
    }

    // Finally we can check if any legends exports are specified
    if !options.legends_exports_to_parse.is_empty() {
        // Parse all legends exports that are specified.
        for legends_export in &options.legends_exports_to_parse {
            let target_path = Path::new(&legends_export);
            // Todo: Add progress helper for legends export parsing
            results.extend(crate::legends_export::parse_legends_export(
                &target_path,
                &options,
            ));
        }
    }

    // Absorb select_creature
    parser::helpers::absorb_select_creature::absorb_select_creature(&mut results);
    // Apply copy_tags_from
    if !options.skip_apply_copy_tags_from {
        parser::helpers::apply_copy_from::apply_copy_tags_from(&mut results);
    }

    results
}

#[cfg(feature = "tauri")]
/// The function `parse_to_json_vec` takes in options and a progress helper, parses the options, and
/// returns a vector of JSON strings.
///
/// Arguments:
///
/// * `options`: A reference to an instance of the `ParserOptions` struct from the `options` module in
/// the crate.
/// * `progress_helper`: A mutable reference to a `ProgressHelper` struct.
///
/// Returns:
///
/// a vector of JSON strings
pub fn parse_to_json_vec(
    options: &crate::options::ParserOptions,
    progress_helper: &mut ProgressHelper,
) -> Vec<String> {
    let raws = parse(options, progress_helper);
    let mut results = Vec::new();

    for raw in raws {
        results.push(serde_json::to_string(&raw).unwrap_or_default());
    }

    results
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
    use tracing::info;

    use crate::parser::raw_locations::RawModuleLocation;

    let mut results: Vec<Box<dyn RawObject>> = Vec::new();
    let module_location = RawModuleLocation::from_path(&location_path);
    let location_path: std::path::PathBuf = location_path.as_ref().to_path_buf();
    // Get a list of all subdirectories in the location
    let raw_modules_in_location: Vec<DirEntry> =
        util::subdirectories(location_path).unwrap_or_default();
    info!(
        "parse_location: Found {} raw modules in {:?}",
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
#[allow(clippy::too_many_lines)]
fn parse_module<P: AsRef<Path>>(
    module_path: &P,
    options: &crate::options::ParserOptions,
    progress_helper: &mut ProgressHelper,
) -> Vec<Box<dyn RawObject>> {
    use tracing::{debug, info, warn};

    // Get information from the module info file
    let module_info_file_path = module_path.as_ref().join("info.txt");
    let Some(module_info_file) = crate::parse_module_info_file_direct(&module_info_file_path)
    else {
        warn!(
            "parse_module: No info.txt found in {:?}",
            module_path.as_ref().file_name().unwrap_or_default(),
        );
        return Vec::new();
    };

    info!(
        "parse_module: Parsing raws for {} v{}",
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
    let mut parse_graphics = true;

    if !objects_path.exists() {
        warn!(
            "parse_module: No objects directory found in {:?}",
            module_path.as_ref().file_name().unwrap_or_default(),
        );
        parse_objects = false;
    }

    if parse_objects && !objects_path.is_dir() {
        warn!(
            "parse_module: Objects directory in {:?} is not a directory",
            module_path.as_ref().file_name().unwrap_or_default(),
        );
        parse_objects = false;
    }

    if !graphics_path.exists() {
        debug!(
            "parse_module: No graphics directory found in {:?}",
            module_path.as_ref().file_name().unwrap_or_default(),
        );
        parse_graphics = false;
    }

    if parse_graphics && !graphics_path.is_dir() {
        warn!(
            "parse_module: Graphics directory in {:?} is not a directory",
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
                    progress_helper.add_to_running_total(results.len());
                }
            }
        }
    }

    // Parse the graphics
    if parse_graphics {
        for entry in walkdir::WalkDir::new(graphics_path)
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
                    progress_helper.add_to_running_total(results.len());
                }
            }
        }
    }

    results
}
