#[cfg(feature = "tauri")]
extern crate tauri;
#[cfg(feature = "tauri")]
use super::structs::ProgressHelper;

use crate::{
    helpers::{absorb_select_creature, apply_copy_tags_from},
    legends_export, parser, util, ModuleInfoFile, ParseResult, ParserError, RawModuleLocation,
    RawObject,
};
use std::path::{Path, PathBuf};
use tracing::{debug, info, warn};
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
) -> Result<crate::ParseResult, crate::ParserError> {
    let options = util::validate_options(options)?;

    let mut results = ParseResult {
        raws: Vec::new(),
        info_files: Vec::new(),
    };

    // Locations can only contain the predefined locations.
    if !options.locations_to_parse.is_empty() {
        let target_path = Path::new(&options.dwarf_fortress_directory);

        // Build paths for each location
        let data_path = target_path.join("data");
        let vanilla_path = data_path.join("vanilla");
        let installed_mods_path = data_path.join("installed_mods");
        let workshop_mods_path = target_path.join("mods");

        // Parse each location
        if options
            .locations_to_parse
            .contains(&RawModuleLocation::Vanilla)
        {
            info!("Dispatching parse for vanilla raws");
            results
                .raws
                .extend(parse_location(&vanilla_path, &options, progress_helper)?);
        }
        if options
            .locations_to_parse
            .contains(&RawModuleLocation::InstalledMods)
        {
            info!("Dispatching parse for installed mods");
            results.raws.extend(parse_location(
                &installed_mods_path,
                &options,
                progress_helper,
            )?);
        }
        if options
            .locations_to_parse
            .contains(&RawModuleLocation::Mods)
        {
            info!("Dispatching parse for workshop/downloaded mods");
            results.raws.extend(parse_location(
                &workshop_mods_path,
                &options,
                progress_helper,
            )?);
        }
    }

    if !options.raw_modules_to_parse.is_empty() {
        // Loop through over module and parse it.
        for raw_module in &options.raw_modules_to_parse {
            let target_path = Path::new(&raw_module);

            // Check for info.txt
            let info_txt_path = target_path.join("info.txt");
            if info_txt_path.exists() {
                info!(
                    "Dispatching parse for module {:?}",
                    target_path.file_name().unwrap_or_default()
                );
                results
                    .raws
                    .extend(parse_module(&target_path, &options, progress_helper)?);
            }
        }
    }

    // Next we can check if any raw files are specified
    if !options.raw_files_to_parse.is_empty() {
        // Parse all raw files that are specified.
        for raw_file in &options.raw_files_to_parse {
            let target_path = Path::new(&raw_file);
            info!(
                "Dispatching parse for raw file {:?}",
                target_path.file_name().unwrap_or_default()
            );
            progress_helper.update_current_task(
                format!(
                    "Parsing raw file {:?}",
                    target_path.file_name().unwrap_or_default()
                )
                .as_str(),
            );
            results
                .raws
                .extend(parser::parse_raw_file(&target_path, &options)?);
        }
    }

    // Finally we can check if any legends exports are specified
    if !options.legends_exports_to_parse.is_empty() {
        // Parse all legends exports that are specified.
        for legends_export in &options.legends_exports_to_parse {
            let target_path = Path::new(&legends_export);

            info!(
                "Dispatching parse for legends export {:?}",
                target_path.file_name().unwrap_or_default()
            );

            progress_helper.update_current_task(
                format!(
                    "Parsing legends export {:?}",
                    target_path.file_name().unwrap_or_default()
                )
                .as_str(),
            );

            results
                .raws
                .extend(legends_export::parse(&target_path, &options)?);
        }
    }

    // Absorb select_creature
    absorb_select_creature(&mut results.raws);
    // Apply copy_tags_from
    if !options.skip_apply_copy_tags_from {
        apply_copy_tags_from(&mut results.raws);
    }

    // Parse the info modules
    progress_helper.update_current_task("Parsing 'info.txt' files");
    results.info_files = crate::parse_module_info_files(&options)?;

    Ok(results)
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
) -> Result<Vec<Box<dyn RawObject>>, ParserError> {
    let mut results: Vec<Box<dyn RawObject>> = Vec::new();
    let location_path: PathBuf = location_path.as_ref().to_path_buf();

    progress_helper.update_current_location(format!("{:?}", &location_path).as_str());

    // Get a list of all subdirectories in the location
    let raw_modules_in_location: Vec<DirEntry> = util::subdirectories(location_path)?;

    info!(
        "Found {} raw modules in {:?}",
        raw_modules_in_location.len(),
        options.locations_to_parse.first().unwrap(),
    );

    // Calculate total number of modules we will parse:
    progress_helper.add_steps(raw_modules_in_location.len());

    // Loop over each module and parse it
    for raw_module in raw_modules_in_location {
        results.extend(parse_module(&raw_module.path(), options, progress_helper)?);
    }

    Ok(results)
}

#[cfg(feature = "tauri")]
#[allow(clippy::too_many_lines)]
fn parse_module<P: AsRef<Path>>(
    module_path: &P,
    options: &crate::options::ParserOptions,
    progress_helper: &mut ProgressHelper,
) -> Result<Vec<Box<dyn RawObject>>, ParserError> {
    // Get information from the module info file
    let module_info_file_path = module_path.as_ref().join("info.txt");
    let module_info_file: ModuleInfoFile;
    match crate::parse_module_info_file_direct(&module_info_file_path) {
        Ok(info_file) => {
            module_info_file = info_file;
        }
        Err(e) => {return Err(e);},
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
        return Ok(Vec::new());
    }

    let mut results: Vec<Box<dyn RawObject>> = Vec::new();

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
                    results.extend(parser::parse_raw_file(&file_path, options)?);
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
                    results.extend(parser::parse_raw_file(&file_path, options)?);
                    progress_helper.add_to_running_total(results.len());
                }
            }
        }
    }

    Ok(results)
}
