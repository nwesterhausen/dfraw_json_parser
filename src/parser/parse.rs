// Shared parse function?

use std::path::{Path, PathBuf};

use walkdir::{DirEntry, WalkDir};

use crate::{
    absorb_select_creature, apply_copy_tags_from, options_has_valid_paths,
    parse_module_info_file_direct, parse_raws_from_single_file, subdirectories,
    tauri_lib::ProgressHelper, ParserOptions, ParsingJob, ProgressPayload, RawModuleLocation,
    RawObject,
};

#[allow(clippy::too_many_lines)]
/// The main parsing function. Follows these steps:
///
/// 1. Validate options
/// 2. Parse each location
/// 3. Parse each module in each location
/// 4. Parse each raw file in each module
/// 5. Apply `absorb_select_creature`
/// 6. Apply `apply_copy_tags_from`
///
/// If the `tauri` feature is enabled, this function will also send progress updates to the
/// `ProgressHelper`. In all the sub-parsing that is done, a `ProgressPayload` struct is used to track progress
/// and send updates to the `ProgressHelper`.
///
/// # Parameters
///
/// * `options`: The `ParserOptions` struct to use when parsing.
/// * `progress_helper`: An optional `ProgressHelper` struct to use when sending progress updates.
///
/// # Returns
///
/// A `Vec` of `Box<dyn RawObject>` objects.
pub fn parse(
    options: &ParserOptions,
    progress_helper: Option<&ProgressHelper>,
) -> Vec<Box<dyn RawObject>> {
    // Guard against invalid path
    if !options_has_valid_paths(options) {
        log::error!(
            "Returning early for bad path. Provided options:\n{:#?}",
            options
        );
        return Vec::new();
    }
    let target_path = Path::new(&options.target_path);
    let mut results: Vec<Box<dyn RawObject>> = Vec::new();

    let mut progress_payload = ProgressPayload::default();

    #[cfg(feature = "tauri")]
    {
        if let Some(progress_helper) = progress_helper {
            progress_payload.update_current_task("Parsing starting");
            progress_helper.send(&progress_payload);
        }
    }

    match options.job {
        ParsingJob::AllModulesInLocations => {
            // Set file paths for each location
            let data_path = target_path.join("data");
            let vanilla_path = data_path.join("vanilla");
            let installed_mods_path = data_path.join("installed_mods");
            let workshop_mods_path = target_path.join("mods");

            let mut location_results: Vec<Box<dyn RawObject>>;

            // Parse each location
            if options
                .locations_to_parse
                .contains(&RawModuleLocation::Vanilla)
            {
                (location_results, progress_payload) = parse_location(
                    &vanilla_path,
                    options,
                    progress_helper,
                    progress_payload.clone(),
                );
                results.extend(location_results);
            }
            #[cfg(feature = "tauri")]
            {
                if let Some(progress_helper) = progress_helper {
                    progress_payload.update_current_task("Parsing workshop mods");
                    progress_helper.send(&progress_payload);
                }
            }
            if options
                .locations_to_parse
                .contains(&RawModuleLocation::InstalledMods)
            {
                (location_results, progress_payload) = parse_location(
                    &installed_mods_path,
                    options,
                    progress_helper,
                    progress_payload.clone(),
                );
                results.extend(location_results);
            }
            #[cfg(feature = "tauri")]
            {
                if let Some(progress_helper) = progress_helper {
                    progress_payload.update_current_task("Parsing workshop mods");
                    progress_helper.send(&progress_payload);
                }
            }
            if options
                .locations_to_parse
                .contains(&RawModuleLocation::Mods)
            {
                (location_results, progress_payload) = parse_location(
                    &workshop_mods_path,
                    options,
                    progress_helper,
                    progress_payload.clone(),
                );
                results.extend(location_results);
            }
            #[cfg(feature = "tauri")]
            {
                if let Some(progress_helper) = progress_helper {
                    progress_payload.update_current_task("Parsing workshop mods");
                    progress_helper.send(&progress_payload);
                }
            }
        }
        ParsingJob::SingleModule => {
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

                return Vec::new();
            }
            let (module_results, _progress_helper) = parse_module(
                &target_path,
                options,
                progress_helper,
                progress_payload.clone(),
            );
            results.extend(module_results);
        }
        ParsingJob::SingleRaw => {
            // The provided path should be a raw file directly
            results.extend(parse_raws_from_single_file(&target_path, options));
        }
        ParsingJob::SingleModuleInfoFile | ParsingJob::AllModuleInfoFiles => {
            // The provided path should be the info.txt file for a module
            log::warn!(
                "Unable to parse info.txt file in this dispatch. Provided options:\n{:#?}",
                options
            );
            return Vec::new();
        }
    }

    // Absorb select_creature
    absorb_select_creature(&mut results);
    // Apply copy_tags_from
    if !options.skip_apply_copy_tags_from {
        apply_copy_tags_from(&mut results);
    }

    #[cfg(feature = "tauri")]
    {
        if let Some(progress_helper) = progress_helper {
            progress_payload.set_final("Parsing Finished");
            progress_helper.send(&progress_payload);
        }
    }

    results
}

#[allow(clippy::too_many_lines)]
/// Parses a single location. Follows these steps:
///
/// 1. Get a list of all subdirectories in the location
/// 2. Parse each module in the location
///
/// If the `tauri` feature is enabled, this function will also send progress updates to the
/// `ProgressHelper`.
///
/// # Parameters
///
/// * `location_path`: The path of the location to parse.
/// * `options`: The `ParserOptions` struct to use when parsing.
/// * `progress_helper`: An optional `ProgressHelper` struct to use when sending progress updates.
/// * `progress_payload`: A `ProgressPayload` struct to use when sending progress updates.
///
/// # Returns
///
/// A tuple containing a `Vec` of `Box<dyn RawObject>` objects and a `ProgressPayload` struct.
fn parse_location<P: AsRef<Path>>(
    location_path: &P,
    options: &ParserOptions,
    progress_helper: Option<&ProgressHelper>,
    mut progress_payload: ProgressPayload,
) -> (Vec<Box<dyn RawObject>>, ProgressPayload) {
    let mut results: Vec<Box<dyn RawObject>> = Vec::new();
    let location_path: PathBuf = location_path.as_ref().to_path_buf();
    let module_location = RawModuleLocation::from_path(&location_path);

    // Get a list of all subdirectories in the location
    let raw_modules_in_location: Vec<DirEntry> = subdirectories(location_path).unwrap_or_default();

    let mut module_results: Vec<Box<dyn RawObject>>;

    log::info!(
        "Found {} raw modules in {:?}",
        raw_modules_in_location.len(),
        options.locations_to_parse.first().unwrap(),
    );

    #[cfg(feature = "tauri")]
    {
        if let Some(progress_helper) = progress_helper {
            progress_payload.update_current_task(&format!(
                "Parsing {} raw modules in {:?}",
                raw_modules_in_location.len(),
                module_location,
            ));
            progress_payload.update_current_location(format!("{module_location:?}").as_str());

            // Calculate total number of modules we will parse:
            progress_payload.add_steps(raw_modules_in_location.len());

            progress_helper.send(&progress_payload);
        }
    }

    // Loop over each module and parse it
    for raw_module in raw_modules_in_location {
        (module_results, progress_payload) = parse_module(
            &raw_module.path(),
            options,
            progress_helper,
            progress_payload.clone(),
        );
        results.extend(module_results);
    }

    (results, progress_payload)
}

#[allow(clippy::too_many_lines)]
/// Parses a single module. Follows these steps:
///
/// 1. Get information from the module info file
/// 2. Get a list of all raw files in the module
/// 3. Parse each raw file in the module
///
/// If the `tauri` feature is enabled, this function will also send progress updates to the
/// `ProgressHelper`.
///
/// # Parameters
///
/// * `module_path`: The path of the module to parse.
/// * `options`: The `ParserOptions` struct to use when parsing.
/// * `progress_helper`: An optional `ProgressHelper` struct to use when sending progress updates.
/// * `progress_payload`: A `ProgressPayload` struct to use when sending progress updates.
///
/// # Returns
///
/// A tuple containing a `Vec` of `Box<dyn RawObject>` objects and a `ProgressPayload` struct.
fn parse_module<P: AsRef<Path>>(
    module_path: &P,
    options: &ParserOptions,
    progress_helper: Option<&ProgressHelper>,
    mut progress_payload: ProgressPayload,
) -> (Vec<Box<dyn RawObject>>, ProgressPayload) {
    // Get information from the module info file
    let module_info_file_path = module_path.as_ref().join("info.txt");
    let module_info_file = parse_module_info_file_direct(&module_info_file_path);

    log::info!(
        "draw_json_parser: Parsing raws for {} v{}",
        module_info_file.get_identifier(),
        module_info_file.get_version(),
    );

    #[cfg(feature = "tauri")]
    {
        if let Some(_progress_helper) = progress_helper {
            progress_payload.update_current_module(
                format!(
                    "{} v{}",
                    module_info_file.get_identifier(),
                    module_info_file.get_version()
                )
                .as_str(),
            );
        }
    }

    // Get a list of all raw files in the module
    let objects_path = module_path.as_ref().join("objects");
    let graphics_path = module_path.as_ref().join("graphics");

    let mut parse_objects = true;
    let mut parse_graphics = true;

    if !objects_path.exists() {
        log::debug!(
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
        log::debug!(
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
        return (Vec::new(), progress_payload);
    }

    let mut results = Vec::new();

    // Parse the objects
    if parse_objects {
        for entry in WalkDir::new(objects_path)
            .into_iter()
            .filter_map(std::result::Result::ok)
        {
            if entry.file_type().is_file() {
                let file_path = entry.path();
                let file_name = file_path.file_name().unwrap_or_default();
                let file_name_str = file_name.to_str().unwrap_or_default();

                if Path::new(file_name_str)
                    .extension()
                    .map_or(false, |ext| ext.eq_ignore_ascii_case("txt"))
                {
                    #[cfg(feature = "tauri")]
                    {
                        if let Some(progress_helper) = progress_helper {
                            progress_payload.add_steps(1);
                            progress_payload.set_current_file(file_name_str);
                            progress_helper.send(&progress_payload);
                        }
                    }
                    let rawfile_results = parse_raws_from_single_file(&file_path, options);
                    #[cfg(feature = "tauri")]
                    {
                        if let Some(_progress_helper) = progress_helper {
                            progress_payload.add_to_running_total(rawfile_results.len());
                        }
                    }
                    results.extend(rawfile_results);
                }
            }
        }
    }

    // Parse the graphics
    if parse_graphics {
        for entry in WalkDir::new(graphics_path)
            .into_iter()
            .filter_map(std::result::Result::ok)
        {
            if entry.file_type().is_file() {
                let file_path = entry.path();
                let file_name = file_path.file_name().unwrap_or_default();
                let file_name_str = file_name.to_str().unwrap_or_default();

                if Path::new(file_name_str)
                    .extension()
                    .map_or(false, |ext| ext.eq_ignore_ascii_case("txt"))
                {
                    #[cfg(feature = "tauri")]
                    {
                        if let Some(progress_helper) = progress_helper {
                            progress_payload.add_steps(1);
                            progress_payload.set_current_file(file_name_str);
                            progress_helper.send(&progress_payload);
                        }
                    }
                    let rawfile_results = parse_raws_from_single_file(&file_path, options);
                    #[cfg(feature = "tauri")]
                    {
                        if let Some(_progress_helper) = progress_helper {
                            progress_payload.add_to_running_total(rawfile_results.len());
                        }
                    }
                    results.extend(rawfile_results);
                }
            }
        }
    }
    #[cfg(feature = "tauri")]
    {
        if let Some(progress_helper) = progress_helper {
            progress_payload.set_final("Parsing Finished");
            progress_helper.send(&progress_payload);
        }
    }
    (results, progress_payload)
}
