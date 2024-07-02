#[cfg(feature = "tauri")]
use super::ProgressHelper;

use std::path::{Path, PathBuf};
use tracing::{debug, info};
use walkdir::DirEntry;

/// Parse a directory of raws, and return a JSON string of the parsed raws. While parsing, this will
/// emit tauri events to the supplied window. The event is titled `PROGRESS` and it uses the `ProgressPayload`
/// payload for the payload.
///
/// The payload supplies the current progress as a float and the name of the current folder being parsed.
///
/// # Parameters
///
/// * `df_game_path`: The path to the Dwarf Fortress install directory
/// * `window`: A `tauri::Window` to emit `PROGRESS` events to.
///
/// # Returns
///
/// A parsing result or a parser error.
///
/// # Errors
///
/// This function will return an error if the Dwarf Fortress directory is invalid, or if the raws cannot be parsed.
#[cfg(feature = "tauri")]
#[allow(clippy::too_many_lines)]
pub(crate) fn parse(
    options: &crate::options::ParserOptions,
    progress_helper: &mut ProgressHelper,
) -> Result<crate::ParseResult, crate::ParserError> {
    // Guard against invalid paths (validate the options)

    use dfraw_parser::utilities::validate_options;

    let options = validate_options(options)?;

    let mut results = ParseResult {
        raws: Vec::new(),
        info_files: Vec::new(),
    };
    let mut unprocessed_raws: Vec<UnprocessedRaw> = Vec::new();

    // Locations can only contain the predefined locations.
    if !options.locations_to_parse.is_empty() {
        let target_path = Path::new(&options.dwarf_fortress_directory);
        progress_helper.reset_details();

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
            progress_helper.set_location(RawModuleLocation::Vanilla);
            info!("Dispatching parse for vanilla raws");
            let parsed_raws = parse_location(&vanilla_path, &options, progress_helper)?;
            results.raws.extend(parsed_raws.parsed_raws);
            unprocessed_raws.extend(parsed_raws.unprocessed_raws);
        }
        if options
            .locations_to_parse
            .contains(&RawModuleLocation::InstalledMods)
        {
            progress_helper.set_location(RawModuleLocation::InstalledMods);
            info!("Dispatching parse for installed mods");
            let parsed_raws = parse_location(&installed_mods_path, &options, progress_helper)?;
            results.raws.extend(parsed_raws.parsed_raws);
            unprocessed_raws.extend(parsed_raws.unprocessed_raws);
        }
        if options
            .locations_to_parse
            .contains(&RawModuleLocation::Mods)
        {
            progress_helper.set_location(RawModuleLocation::Mods);
            info!("Dispatching parse for workshop/downloaded mods");
            let parsed_raws = parse_location(&workshop_mods_path, &options, progress_helper)?;
            results.raws.extend(parsed_raws.parsed_raws);
            unprocessed_raws.extend(parsed_raws.unprocessed_raws);
        }
    }

    if !options.raw_modules_to_parse.is_empty() {
        progress_helper.reset_details();
        progress_helper.set_task(ProgressTask::ParseModuleInfoFiles);

        // Loop through over module and parse it.
        for raw_module in &options.raw_modules_to_parse {
            let target_path = Path::new(&raw_module);

            // Check for info.txt
            let info_txt_path = target_path.join("info.txt");
            if info_txt_path.exists() {
                progress_helper.set_module(
                    target_path
                        .file_name()
                        .unwrap_or_default()
                        .to_str()
                        .unwrap_or_default(),
                );
                info!(
                    "Dispatching parse for module {:?}",
                    target_path.file_name().unwrap_or_default()
                );
                let parsed_raws = parse_module(&target_path, &options, progress_helper)?;
                results.raws.extend(parsed_raws.parsed_raws);
                unprocessed_raws.extend(parsed_raws.unprocessed_raws);
            }
        }
    }

    // Next we can check if any raw files are specified
    if !options.raw_files_to_parse.is_empty() {
        progress_helper.reset_details();
        progress_helper.set_task(ProgressTask::ParseRaws);

        // Parse all raw files that are specified.
        for raw_file in &options.raw_files_to_parse {
            let target_path = Path::new(&raw_file);
            progress_helper.set_file(
                target_path
                    .file_name()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap_or_default(),
            );
            info!(
                "Dispatching parse for raw file {:?}",
                target_path.file_name().unwrap_or_default()
            );
            let parse_result = parser::parse_raw_file(&target_path, &options)?;
            results.raws.extend(parse_result.parsed_raws);
            unprocessed_raws.extend(parse_result.unprocessed_raws);
        }
    }

    // Finally we can check if any legends exports are specified
    if !options.legends_exports_to_parse.is_empty() {
        progress_helper.reset_details();
        progress_helper.set_task(ProgressTask::ParseLegends);

        // Parse all legends exports that are specified.
        for legends_export in &options.legends_exports_to_parse {
            let target_path = Path::new(&legends_export);
            progress_helper.set_file(
                target_path
                    .file_name()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap_or_default(),
            );
            info!(
                "Dispatching parse for legends export {:?}",
                target_path.file_name().unwrap_or_default()
            );
            results
                .raws
                .extend(legends_export::parse(&target_path, &options)?);
        }
    }

    progress_helper.reset_details();
    progress_helper.set_task(ProgressTask::ResolveRaws);

    // Resolve the unprocessed creatures
    // Prerequisites: build a list of creature variations
    let creature_variations: Vec<CreatureVariation> = results
        .raws
        .iter()
        .filter_map(|raw| {
            if raw.get_type() == &crate::ObjectType::CreatureVariation {
                if let Some(cv) = raw
                    .as_ref()
                    .as_any()
                    .downcast_ref::<CreatureVariation>()
                    .cloned()
                {
                    return Some(cv);
                }
                tracing::error!(
                    "Matched CreatureVariation but failed to downcast for {}",
                    raw.get_identifier()
                );
            }
            None
        })
        .collect();

    info!(
        "Resolving {} unprocessed creatures using {} creature variation definitions",
        unprocessed_raws.len(),
        creature_variations.len()
    );

    // Write the unprocessed raws to a file
    // let _ = serde_json::to_writer_pretty(
    //     std::fs::File::create("unprocessed_raws.json").unwrap(),
    //     &unprocessed_raws,
    // );

    let mut simple_unprocessed: Vec<UnprocessedRaw> = Vec::new();
    let mut complex_unprocessed: Vec<UnprocessedRaw> = Vec::new();

    // Split the unprocessed raws into simple and complex
    for unprocessed_raw in unprocessed_raws {
        if unprocessed_raw.is_simple() {
            simple_unprocessed.push(unprocessed_raw);
        } else {
            complex_unprocessed.push(unprocessed_raw);
        }
    }

    // Resolve the simple creatures first
    let resolved_simple_creatures: Vec<crate::creature::Creature> = simple_unprocessed
        .iter_mut()
        .filter(|raw| raw.raw_type() == crate::ObjectType::Creature)
        .filter_map(|raw| {
            match raw.resolve(creature_variations.as_slice(), results.raws.as_slice()) {
                Ok(c) => Some(c),
                Err(e) => {
                    tracing::error!(
                        "Unable to resolve simple creature {}: {:?}",
                        raw.get_identifier(),
                        e
                    );
                    None
                }
            }
        })
        .map(|c| crate::helpers::clone_raw_object_box(&c))
        .filter_map(|c| {
            if let Some(creature) = c
                .as_ref()
                .as_any()
                .downcast_ref::<crate::creature::Creature>()
            {
                Some(creature.clone())
            } else {
                tracing::error!("Downcast failed for simple creature {}", c.get_identifier());
                None
            }
        })
        .collect();

    info!(
        "Resolved {} simple creatures",
        resolved_simple_creatures.len()
    );

    results.raws.extend(
        resolved_simple_creatures
            .iter()
            .map(|c| Box::new(c.clone()) as Box<dyn RawObject>),
    );

    // Now we can do the second pass through the unprocessed creatures, but add the complex creatures
    // to the results.raws vector as they are resolved.
    let mut resolved_complex_creatures = 0_usize;
    for unprocessed_raw in &mut complex_unprocessed {
        if unprocessed_raw.raw_type() == crate::ObjectType::Creature {
            match unprocessed_raw.resolve(creature_variations.as_slice(), results.raws.as_slice()) {
                Ok(c) => {
                    resolved_complex_creatures += 1;
                    results.raws.push(crate::helpers::clone_raw_object_box(&c));
                }
                Err(e) => {
                    tracing::error!(
                        "Unable to resolve complex creature {}: {:?}",
                        unprocessed_raw.get_identifier(),
                        e
                    );
                }
            }
        }
    }

    info!("Resolved {resolved_complex_creatures} complex creatures");

    if options.log_summary {
        let summary = crate::util::summarize_raws(&results.raws);
        progress_helper.send_summary(&summary);
        crate::util::log_summary(&summary);
    }

    progress_helper.set_task(ProgressTask::ParseModuleInfoFiles);
    results.info_files = crate::parse_module_info_files(&options)?;

    progress_helper.finalize_and_send();

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
///
/// # Errors
///
/// This function will return an error if the location path is not a directory.
fn parse_location<P: AsRef<Path>>(
    location_path: &P,
    options: &crate::options::ParserOptions,
    progress_helper: &mut ProgressHelper,
) -> Result<crate::FileParseResults, ParserError> {
    let mut results: Vec<Box<dyn RawObject>> = Vec::new();
    let mut unprocessed_raws: Vec<crate::unprocessed_raw::UnprocessedRaw> = Vec::new();
    let location_path: PathBuf = location_path.as_ref().to_path_buf();

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
        match parse_module(&raw_module.path(), options, progress_helper) {
            Ok(module_results) => {
                results.extend(module_results.parsed_raws);
                unprocessed_raws.extend(module_results.unprocessed_raws);
            }
            Err(e) => {
                debug!("Skipping parsing module: {:?}", e);
            }
        }
    }

    Ok(crate::FileParseResults {
        parsed_raws: results,
        unprocessed_raws,
    })
}

#[cfg(feature = "tauri")]
#[allow(clippy::too_many_lines)]
fn parse_module<P: AsRef<Path>>(
    module_path: &P,
    options: &ParserOptions,
    progress_helper: &mut ProgressHelper,
) -> Result<crate::FileParseResults, ParserError> {
    // Get information from the module info file
    let module_info_file_path = module_path.as_ref().join("info.txt");
    let module_info_file: ModuleInfoFile =
        match crate::parse_module_info_file_direct(&module_info_file_path) {
            Ok(info_file) => info_file,
            Err(e) => {
                return Err(e);
            }
        };

    progress_helper.set_module(
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
        debug!(
            "parse_module: No objects directory found in {:?}",
            module_path.as_ref().file_name().unwrap_or_default(),
        );
        parse_objects = false;
    }

    if parse_objects && !objects_path.is_dir() {
        debug!(
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
        debug!(
            "parse_module: Graphics directory in {:?} is not a directory",
            module_path.as_ref().file_name().unwrap_or_default(),
        );
        parse_graphics = false;
    }

    // Exit early if nothing to parse
    if !parse_graphics && !parse_objects {
        return Ok(crate::FileParseResults {
            parsed_raws: vec![],
            unprocessed_raws: vec![],
        });
    }

    let mut results: Vec<Box<dyn RawObject>> = Vec::new();
    let mut unprocessed_raws: Vec<crate::unprocessed_raw::UnprocessedRaw> = Vec::new();

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
                    progress_helper.set_file(file_name_str);
                    progress_helper.set_file_location(file_path.to_str().unwrap_or_default());
                    progress_helper.advance_and_send_update();

                    match parser::parse_raw_file(&file_path, options) {
                        Ok(mut objects_results) => {
                            progress_helper.add_to_running_total(
                                objects_results.parsed_raws.len()
                                    + objects_results.unprocessed_raws.len(),
                            );
                            results.append(&mut objects_results.parsed_raws);
                            unprocessed_raws.append(&mut objects_results.unprocessed_raws);
                        }
                        Err(e) => {
                            debug!("Skipping parsing objects file: {:?}", e);
                        }
                    }
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
                    progress_helper.set_file(file_name_str);
                    progress_helper.set_file_location(file_path.to_str().unwrap_or_default());
                    progress_helper.advance_and_send_update();

                    match parser::parse_raw_file(&file_path, options) {
                        Ok(mut graphics_results) => {
                            progress_helper
                                .add_to_running_total(graphics_results.parsed_raws.len());
                            results.append(&mut graphics_results.parsed_raws);
                        }
                        Err(e) => {
                            debug!("Skipping parsing graphics file: {:?}", e);
                        }
                    }
                }
            }
        }
    }

    Ok(crate::FileParseResults {
        parsed_raws: results,
        unprocessed_raws,
    })
}
