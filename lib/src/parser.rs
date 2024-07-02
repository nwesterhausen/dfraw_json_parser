use std::path::{Path, PathBuf};

use tracing::{debug, error, info};
use walkdir::{DirEntry, WalkDir};

use crate::{
    legends_export,
    metadata::{ObjectType, ParserOptions, RawModuleLocation},
    reader::{parse_raw_file, FileParseResult, UnprocessedRaw},
    traits::RawObject,
    utilities::{
        clone_raw_object_box, log_summary, subdirectories, summarize_raws, validate_options,
    },
    Creature, CreatureVariation, InfoFile, ParserError,
};

/// A parsing result that contains the parsed raws and info files.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ParseResult {
    /// The parsed raw objects.
    pub raws: Vec<Box<dyn RawObject>>,
    /// The parsed module info files.
    pub info_files: Vec<InfoFile>,
}

#[allow(clippy::too_many_lines)]
/// Given the supplied `ParserOptions`, parse the raws and return a vector of boxed dynamic raw objects.
///
/// Note: This is unable to parse the info.txt file for a module. Use `parse_module_info_file` for that.
///
/// # Arguments
///
/// * `options` - A reference to a `ParserOptions` struct that contains the parsing options.
///
/// # Returns
///
/// A vector of boxed dynamic raw objects.
///
/// # Errors
///
/// * `ParserError::Io` - If we can't read the raws from the Dwarf Fortress directory (various reasons)
/// * `ParserError::InvalidPath` - If the path to the Dwarf Fortress directory is invalid
///
/// Other errors which are returned from the called functions within this function are not propagated, because the
/// only "full" blocker is if the Dwarf Fortress directory is invalid.
///
#[allow(clippy::cognitive_complexity)]
pub fn parse(options: &ParserOptions) -> Result<ParseResult, ParserError> {
    // Guard against invalid paths
    let options = validate_options(options)?;

    let mut results = ParseResult {
        raws: Vec::new(),
        info_files: Vec::new(),
    };
    let mut unprocessed_raws: Vec<UnprocessedRaw> = Vec::new();

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
            let parsed_raws = parse_location(&vanilla_path, &options)?;
            results.raws.extend(parsed_raws.parsed_raws);
            unprocessed_raws.extend(parsed_raws.unprocessed_raws);
        }
        if options
            .locations_to_parse
            .contains(&RawModuleLocation::InstalledMods)
        {
            info!("Dispatching parse for installed mods");
            let parsed_raws = parse_location(&installed_mods_path, &options)?;
            results.raws.extend(parsed_raws.parsed_raws);
            unprocessed_raws.extend(parsed_raws.unprocessed_raws);
        }
        if options
            .locations_to_parse
            .contains(&RawModuleLocation::Mods)
        {
            info!("Dispatching parse for workshop/downloaded mods");
            let parsed_raws = parse_location(&workshop_mods_path, &options)?;
            results.raws.extend(parsed_raws.parsed_raws);
            unprocessed_raws.extend(parsed_raws.unprocessed_raws);
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
                let parsed_raws = parse_module(&target_path, &options)?;
                results.raws.extend(parsed_raws.parsed_raws);
                unprocessed_raws.extend(parsed_raws.unprocessed_raws);
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
            let parsed_raws = parse_raw_file(&target_path, &options)?;
            results.raws.extend(parsed_raws.parsed_raws);
            unprocessed_raws.extend(parsed_raws.unprocessed_raws);
        }
    }

    // Finally we can check if any legends exports are specified
    if !options.legends_exports_to_parse.is_empty() {
        // Parse all legends exports that are specified.
        for legends_export in &options.legends_exports_to_parse {
            let target_path = Path::new(&legends_export);

            results
                .raws
                .extend(legends_export::parse(&target_path, &options)?);
        }
    }

    // Resolve the unprocessed creatures
    // Prerequisites: build a list of creature variations
    let creature_variations: Vec<CreatureVariation> = results
        .raws
        .iter()
        .filter_map(|raw| {
            if raw.get_type() == &ObjectType::CreatureVariation {
                if let Some(cv) = raw
                    .as_ref()
                    .as_any()
                    .downcast_ref::<CreatureVariation>()
                    .cloned()
                {
                    return Some(cv);
                }
                error!(
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
    let resolved_simple_creatures: Vec<Creature> = simple_unprocessed
        .iter_mut()
        .filter(|raw| raw.raw_type() == ObjectType::Creature)
        .filter_map(|raw| {
            match raw.resolve(creature_variations.as_slice(), results.raws.as_slice()) {
                Ok(c) => Some(c),
                Err(e) => {
                    error!(
                        "Unable to resolve simple creature {}: {:?}",
                        raw.get_identifier(),
                        e
                    );
                    None
                }
            }
        })
        .map(|c| clone_raw_object_box(&c))
        .filter_map(|c| {
            c.as_ref().as_any().downcast_ref::<Creature>().map_or_else(
                || {
                    error!("Downcast failed for simple creature {}", c.get_identifier());
                    None
                },
                |creature| Some(creature.clone()),
            )
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
        if unprocessed_raw.raw_type() == ObjectType::Creature {
            match unprocessed_raw.resolve(creature_variations.as_slice(), results.raws.as_slice()) {
                Ok(c) => {
                    resolved_complex_creatures += 1;
                    results.raws.push(clone_raw_object_box(&c));
                }
                Err(e) => {
                    error!(
                        "Unable to resolve complex creature {}: {:?}",
                        unprocessed_raw.get_identifier(),
                        e
                    );
                }
            }
        }
    }

    info!("Resolved {resolved_complex_creatures} complex creatures");

    // Parse the info modules
    results.info_files = parse_module_info_files(&options)?;

    // Print a summary of what we parsed (sum by ObjectType)
    if options.log_summary {
        let summary = summarize_raws(results.raws.as_slice());
        log_summary(&summary);
    }

    Ok(results)
}

/// The function `parse_module_info_files` parses module information files based on the provided options.
///
/// Arguments:
///
/// * `options`: A reference to a `ParserOptions` struct, which contains various options for parsing
/// module information.
///
/// Returns:
///
/// The function `parse_module_info_files` returns a `Vec<InfoFile>`.
fn parse_module_info_files(options: &ParserOptions) -> Result<Vec<InfoFile>, ParserError> {
    let mut results = Vec::new();

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
            results.extend(parse_module_info_files_at_location(&vanilla_path)?);
        }
        if options
            .locations_to_parse
            .contains(&RawModuleLocation::InstalledMods)
        {
            results.extend(parse_module_info_files_at_location(&installed_mods_path)?);
        }
        if options
            .locations_to_parse
            .contains(&RawModuleLocation::Mods)
        {
            results.extend(parse_module_info_files_at_location(&workshop_mods_path)?);
        }
    }

    // Parse any raw modules that are specified
    if !options.raw_modules_to_parse.is_empty() {
        // Parse all raw modules that are specified.
        for raw_module_path in options.raw_modules_to_parse.as_slice() {
            results.push(parse_module_info_file_in_module(raw_module_path)?);
        }
    }

    // Parse any module info files that are specified directly
    if !options.module_info_files_to_parse.is_empty() {
        // Parse all module info files that are specified.
        for module_info_file_path in options.module_info_files_to_parse.as_slice() {
            results.push(parse_module_info_file_direct(module_info_file_path)?);
        }
    }

    Ok(results)
}

/// Parse the `info.txt` file at the `module_path` provided. Returns a `InfoFile` if successful.
///
/// Arguments:
///
/// * `module_path`: A reference to a path that points to the module directory.
///
/// Returns:
///
/// A `InfoFile` or `ParserError`
///
/// ## Errors
///
/// * `ParserError::Io` - If the `info.txt` file cannot be read, doesn't exist, or is an invalid `info.txt` file
fn parse_module_info_file_in_module<P: AsRef<Path>>(
    module_path: &P,
) -> Result<InfoFile, ParserError> {
    let module_path: PathBuf = module_path.as_ref().to_path_buf();
    let module_info_file_path = module_path.join("info.txt");
    parse_module_info_file_direct(&module_info_file_path)
}

/// Parses the raws in the provided location path, and returns a vector of boxed dynamic raw objects.
///
/// # Arguments
///
/// * `location_path` - A reference to the path to parse.
/// * `options` - A reference to a `ParserOptions` struct that contains the parsing options.
///
/// # Returns
///
/// A vector of boxed dynamic raw objects.
///
/// # Errors
///
/// * `ParserError::Io` - If we can't read the raws from the Dwarf Fortress directory (various reasons)
fn parse_location<P: AsRef<Path>>(
    location_path: &P,
    options: &ParserOptions,
) -> Result<FileParseResult, ParserError> {
    let mut results: Vec<Box<dyn RawObject>> = Vec::new();
    let mut unprocessed_raws: Vec<UnprocessedRaw> = Vec::new();

    let location_path: PathBuf = location_path.as_ref().to_path_buf();
    // Get a list of all subdirectories in the location
    let raw_modules_in_location: Vec<DirEntry> = subdirectories(location_path)?;

    info!(
        "Found {} raw modules in {:?}",
        raw_modules_in_location.len(),
        options
            .locations_to_parse
            .first()
            .unwrap_or(&RawModuleLocation::Unknown)
    );

    // Loop over each module and parse it
    for raw_module in raw_modules_in_location {
        match parse_module(&raw_module.path(), options) {
            Ok(module_results) => {
                results.extend(module_results.parsed_raws);
                unprocessed_raws.extend(module_results.unprocessed_raws);
            }
            Err(e) => {
                debug!("Skipping parsing module: {:?}", e);
            }
        }
    }

    Ok(FileParseResult {
        parsed_raws: results,
        unprocessed_raws,
    })
}

/// The function `parse_module_info_files_at_location` takes a location path as input, retrieves a list
/// of subdirectories at that location, and parses each subdirectory's "info.txt" file into a
/// `InfoFile` struct, returning a vector of these structs.
///
/// # Arguments:
///
/// * `location_path`: the path to the directory where the module info files are.
///
/// # Returns:
///
/// The function `parse_module_info_files_at_location` returns a vector of `InfoFile` objects.
///
/// # Errors
///
/// * `ParserError::Io` - If we can't read the `info.txt` file properly
fn parse_module_info_files_at_location<P: AsRef<Path>>(
    location_path: &P,
) -> Result<Vec<InfoFile>, ParserError> {
    let location_path: PathBuf = location_path.as_ref().to_path_buf();

    // Get a list of all subdirectories in the location
    let raw_modules_in_location: Vec<DirEntry> = subdirectories(location_path.clone())?;

    info!(
        "Found {} raw modules in {:?}",
        raw_modules_in_location.len(),
        location_path.file_name().unwrap_or_default(),
    );

    Ok(raw_modules_in_location
        .iter()
        .filter_map(
            |raw_module| match parse_module_info_file_in_module(&raw_module.path()) {
                Ok(info_file) => Some(info_file),
                Err(e) => {
                    debug!("Skipping parsing module info file: {:?}", e);
                    None
                }
            },
        )
        .collect::<Vec<InfoFile>>())
}

/// The function `parse_module_info_file_direct` parses a module info file and returns a
/// `InfoFile` object.
///
/// Arguments:
///
/// * `module_info_file_path`: A reference to a path that points to the module info file.
///
/// Returns:
///
/// The function `parse_module_info_file_direct` returns a `InfoFile` object.
fn parse_module_info_file_direct<P: AsRef<Path>>(
    module_info_file_path: &P,
) -> Result<InfoFile, ParserError> {
    InfoFile::parse(module_info_file_path)
}

#[allow(clippy::too_many_lines)]
/// The `parse_module` function parses raw files from a module directory and returns a vector of parsed
/// objects.
///
/// Arguments:
///
/// * `module_path`: the path to the module directory that contains the raw files to parse.
/// * `options`: The parsing options which determine what and how to parse the raw files.
///
/// Returns:
///
/// The function `parse_module` returns a vector of boxed dynamic objects (`Vec<Box<dyn RawObject>>`).
///
/// # Errors
///
/// * `ParserError::Io` - If we can't read the raws from the Dwarf Fortress directory (various reasons)
fn parse_module<P: AsRef<Path>>(
    module_path: &P,
    options: &ParserOptions,
) -> Result<FileParseResult, ParserError> {
    // Get information from the module info file
    let module_info_file_path = module_path.as_ref().join("info.txt");
    let module_info_file: InfoFile = match parse_module_info_file_direct(&module_info_file_path) {
        Ok(info_file) => info_file,
        Err(e) => {
            return Err(e);
        }
    };

    // Get a list of all raw files in the module
    let objects_path = module_path.as_ref().join("objects");
    let graphics_path = module_path.as_ref().join("graphics");

    let mut parse_objects = true;
    let mut parse_graphics = options
        .object_types_to_parse
        .contains(&ObjectType::Graphics);

    if !objects_path.exists() {
        debug!(
            "Ignoring objects directory in {:?} because it does not exist",
            module_path.as_ref().file_name().unwrap_or_default(),
        );
        parse_objects = false;
    }

    if parse_objects && !objects_path.is_dir() {
        debug!(
            "Ignoring objects directory in {:?} because it is not a directory",
            module_path.as_ref().file_name().unwrap_or_default(),
        );
        parse_objects = false;
    }

    if !graphics_path.exists() {
        debug!(
            "Ignoring graphics directory in {:?} because it does not exist",
            module_path.as_ref().file_name().unwrap_or_default(),
        );
        parse_graphics = false;
    }

    if parse_graphics && !graphics_path.is_dir() {
        debug!(
            "Ignoring graphics directory in {:?} because it is not a directory",
            module_path.as_ref().file_name().unwrap_or_default(),
        );
        parse_graphics = false;
    }

    // Guard against having nothing to parse.
    if !parse_graphics && !parse_objects {
        return Ok(FileParseResult {
            parsed_raws: Vec::new(),
            unprocessed_raws: Vec::new(),
        });
    }

    let mut results: Vec<Box<dyn RawObject>> = Vec::new();
    let mut unprocessed_raws: Vec<UnprocessedRaw> = Vec::new();

    // Parse the objects
    if parse_objects {
        info!(
            "Parsing objects for {} v{}",
            module_info_file.get_identifier(),
            module_info_file.get_version(),
        );
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
                    match parse_raw_file(&file_path, options) {
                        Ok(mut file_parse_results) => {
                            results.append(&mut file_parse_results.parsed_raws);
                            unprocessed_raws.append(&mut file_parse_results.unprocessed_raws);
                        }
                        Err(e) => {
                            debug!("Skipping parsing objects: {:?}", e);
                        }
                    }
                }
            }
        }
    }

    // Parse the graphics
    if parse_graphics {
        info!(
            "Parsing graphics for {} v{}",
            module_info_file.get_identifier(),
            module_info_file.get_version(),
        );
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
                    match parse_raw_file(&file_path, options) {
                        Ok(mut graphics) => {
                            results.append(&mut graphics.parsed_raws);
                            unprocessed_raws.append(&mut graphics.unprocessed_raws);
                        }
                        Err(e) => {
                            debug!("Skipping parsing graphics: {:?}", e);
                        }
                    }
                }
            }
        }
    }

    Ok(FileParseResult {
        parsed_raws: results,
        unprocessed_raws,
    })
}
