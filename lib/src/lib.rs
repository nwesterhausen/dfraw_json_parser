/*!
`dfraw_json_parser` provides a way to turn raw files from Dwarf Fortress into JSON. It's
currently useful for getting some basic information from from a limited set of raw types:

- creatures
- plants
- inorganics (rocks, ores, etc)
- materials
- syndromes
- graphics (sprites and tile pages, some layer support)
- limited template support, creature variations support, and select XX support

## How It Works

It functions using a regular expression to break apart the tokens and then checks the
key in the token against a long list of ones it knows about. Any matches are utilized to
build its own representation of the raw. Optionally you can turn this result into JSON using
the `serde_json` library. Or I guess turn it into anything serde supports.

## Tauri Support

This library was built to provide raw parsing for another project, Overseer's Reference Manual,
which creates a tauri app that lets the user look at the raws on their machine in a searchable
and filterable manner. The "tauri" feature flag enables functions which will emit parsing progress
back to the tauri window.

## Glossary of Terms

### Raw

A raw is a definition used by Dwarf Fortress to create creatures, things and objects in the world.
They can be used to also define how things interact. They are composed of a variety of tokens inside
a raw file, starting with an identifier token of `[#type#:#identifier#]`. Raws can also select and
modify other raws using a `[SELECT:#identifier#]` token.

### Raw File

A raw file is a single text file (with .txt extension) which has the first line as the raw file
identifier, and then defines the type of raw file using the `[OBJECT:#type#]` token. The file can
contain any number of raws of that type.

### Raw File Module, Raw Module

Since Dwarf Fortress 50, raws are organized into directories. (Previously they were in a flat
structure.) This was done to facilitate the steam workshop.

### Raw Module Location, Module Location

Raw file module folders are located in 3 locations that this library recognizes:

- `$df_game_dir/data/vanilla`: the vanilla raws are here
- `$df_game_dir/data/installed_mods`: the workshop mods which have been used in generating a world
are here. They are considered installed because they are in use by at least one world. (Note: these
are never removed even if you delete all worlds which use them.)
- `$df_game_dir/mods`: the workshop mods are stored here to be available when creating a world

### Info Text File

In each raw module, there is an `info.txt` file which contains information about that module, like
author, version, earliest compatible version (for updating in existing saves), and information
for the steam workshop if it is a mod downloaded from the steam workshop.

*/

#![warn(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]

use parser::helpers::{absorb_select_creature, apply_copy_tags_from};
use std::path::{Path, PathBuf};
use tracing::{debug, info, warn};
use util::validate_options;
use walkdir::{DirEntry, WalkDir};

mod errors;
mod legends_export;
mod options;
mod parser;

pub use errors::ParserError;

pub struct ParseResult {
    pub raws: Vec<Box<dyn RawObject>>,
    pub info_files: Vec<ModuleInfoFile>,
}

#[cfg(feature = "tauri")]
/// # Tauri Support
///
/// This has functions for interacting with the Tauri application.
///
/// This provides the same library functions but also allows for sending progress updates to the Tauri application.
///
/// The main functions:
///
/// - `parse`: Parses the raws in the provided location path, and returns a vector of boxed dynamic raw objects.
/// - `parse_to_json`: Parses the raws in the provided location path, and returns a vector of JSON strings.
mod tauri_lib;

/// This has utility functions for file operations and directory traversal.
///
/// The functions provide functionality for working with directories, files, and paths.
/// It includes functions for getting subdirectories of a given directory, retrieving the name of the parent directory,
/// validating game directories, writing raw objects and string vectors to files, and converting raw objects to JSON strings.
///
/// The main functions:
///
/// - `subdirectories`: Retrieves a vector of subdirectories for a given directory.
/// - `get_parent_dir_name`: Retrieves the name of the parent directory for a given path.
/// - `path_from_game_directory`: Validates and returns a `PathBuf` for a given game directory path.
/// - `write_raw_vec_to_file`: Writes a vector of raw objects to a file in JSON format.
/// - `write_json_string_vec_to_file`: Writes a vector of strings to a file, with each string on a separate line.
/// - `options_has_valid_paths`: Validates the paths in the `ParserOptions` struct.
/// - `raws_to_string`: Converts a vector of raw objects to a JSON string representation.
pub mod util;

/// Option struct for passing to any parse function.
pub use options::ParserOptions;
pub use parser::*;

#[cfg(feature = "tauri")]
pub use tauri_lib::ProgressPayload;

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
pub fn parse(options: &ParserOptions) -> Result<ParseResult, ParserError> {
    // Guard against invalid paths
    let options = validate_options(options)?;

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
                .extend(parse_location(&vanilla_path, &options)?);
        }
        if options
            .locations_to_parse
            .contains(&RawModuleLocation::InstalledMods)
        {
            info!("Dispatching parse for installed mods");
            results
                .raws
                .extend(parse_location(&installed_mods_path, &options)?);
        }
        if options
            .locations_to_parse
            .contains(&RawModuleLocation::Mods)
        {
            info!("Dispatching parse for workshop/downloaded mods");
            results
                .raws
                .extend(parse_location(&workshop_mods_path, &options)?);
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
                results.raws.extend(parse_module(&target_path, &options)?);
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
    results.info_files = parse_module_info_files(&options)?;

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
/// The function `parse_module_info_files` returns a `Vec<ModuleInfoFile>`.
fn parse_module_info_files(options: &ParserOptions) -> Result<Vec<ModuleInfoFile>, ParserError> {
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

/// Parse the `info.txt` file at the `module_path` provided. Returns a `ModuleInfoFile` if successful.
///
/// Arguments:
///
/// * `module_path`: A reference to a path that points to the module directory.
///
/// Returns:
///
/// A `ModuleInfoFile` or `ParserError`
///
/// ## Errors
///
/// * `ParserError::Io` - If the `info.txt` file cannot be read, doesn't exist, or is an invalid `info.txt` file
fn parse_module_info_file_in_module<P: AsRef<Path>>(
    module_path: &P,
) -> Result<ModuleInfoFile, ParserError> {
    let module_path: PathBuf = module_path.as_ref().to_path_buf();
    let module_info_file_path = module_path.join("info.txt");
    parse_module_info_file_direct(&module_info_file_path)
}

#[cfg(feature = "tauri")]
/// Parse a directory of raws, and return a JSON string of the parsed raws. While parsing, this will
/// emit tauri events to the supplied window. The event has title `PROGRESS` and uses the `ProgressPayload`
/// object for the payload.
///
/// Set the `options` appropriately for the job you want to perform.
///
/// The payload supplies the current progress as a float and the name of the current folder to parse.
///
/// Properties:
///
/// * `options`: The `ParserOptions` to use for parsing.
/// * `window`: A `tauri::Window` to emit `PROGRESS` events to.
///
/// Returns:
///
/// A JSON string with details on all raws in the game path.
///
/// # Errors
///
/// * `ParserError::Io` - If we can't read the raws from the Dwarf Fortress directory (various reasons)
/// * `ParserError::InvalidPath` - If the path to the Dwarf Fortress directory is invalid
///
/// Other errors which are returned from the called functions within this function are not propagated, because the
/// only "full" blocker is if the Dwarf Fortress directory is invalid.
pub fn parse_with_tauri_emit(
    options: &ParserOptions,
    window: tauri::Window,
) -> Result<ParseResult, ParserError> {
    tauri_lib::parse(options, window)
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
) -> Result<Vec<Box<dyn RawObject>>, ParserError> {
    let mut results: Vec<Box<dyn RawObject>> = Vec::new();
    let location_path: PathBuf = location_path.as_ref().to_path_buf();
    // Get a list of all subdirectories in the location
    let raw_modules_in_location: Vec<DirEntry> = util::subdirectories(location_path)?;

    info!(
        "Found {} raw modules in {:?}",
        raw_modules_in_location.len(),
        options.locations_to_parse.first().unwrap(),
    );

    // Loop over each module and parse it
    for raw_module in raw_modules_in_location {
        match parse_module(&raw_module.path(), options) {
            Ok(module_results) => {
                results.extend(module_results);
            }
            Err(e) => {
                debug!("Skipping parsing module: {:?}", e);
            }
        }
    }

    Ok(results)
}

/// The function `parse_module_info_files_at_location` takes a location path as input, retrieves a list
/// of subdirectories at that location, and parses each subdirectory's "info.txt" file into a
/// `ModuleInfoFile` struct, returning a vector of these structs.
///
/// # Arguments:
///
/// * `location_path`: the path to the directory where the module info files are.
///
/// # Returns:
///
/// The function `parse_module_info_files_at_location` returns a vector of `ModuleInfoFile` objects.
///
/// # Errors
///
/// * `ParserError::Io` - If we can't read the `info.txt` file properly
fn parse_module_info_files_at_location<P: AsRef<Path>>(
    location_path: &P,
) -> Result<Vec<ModuleInfoFile>, ParserError> {
    let location_path: PathBuf = location_path.as_ref().to_path_buf();

    // Get a list of all subdirectories in the location
    let raw_modules_in_location: Vec<DirEntry> = util::subdirectories(location_path.clone())?;

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
                    warn!("Skipping parsing module info file: {:?}", e);
                    None
                }
            },
        )
        .collect::<Vec<ModuleInfoFile>>())
}

/// The function `parse_module_info_file_direct` parses a module info file and returns a
/// `ModuleInfoFile` object.
///
/// Arguments:
///
/// * `module_info_file_path`: A reference to a path that points to the module info file.
///
/// Returns:
///
/// The function `parse_module_info_file_direct` returns a `ModuleInfoFile` object.
fn parse_module_info_file_direct<P: AsRef<Path>>(
    module_info_file_path: &P,
) -> Result<ModuleInfoFile, ParserError> {
    parser::ModuleInfoFile::parse(module_info_file_path)
}

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
fn parse_module<P: AsRef<Path>>(
    module_path: &P,
    options: &ParserOptions,
) -> Result<Vec<Box<dyn RawObject>>, ParserError> {
    // Get information from the module info file
    let module_info_file_path = module_path.as_ref().join("info.txt");
    let module_info_file: ModuleInfoFile =
        match parse_module_info_file_direct(&module_info_file_path) {
            Ok(info_file) => info_file,
            Err(e) => {
                return Err(e);
            }
        };

    // Get a list of all raw files in the module
    let objects_path = module_path.as_ref().join("objects");
    let graphics_path = module_path.as_ref().join("graphics");

    let mut parse_objects = true;
    let mut parse_graphics = options.raws_to_parse.contains(&ObjectType::Graphics);

    if !objects_path.exists() {
        debug!(
            "Ignoring objects directory in {:?} because it does not exist",
            module_path.as_ref().file_name().unwrap_or_default(),
        );
        parse_objects = false;
    }

    if parse_objects && !objects_path.is_dir() {
        warn!(
            "Ignoring objects directory in {:?} because it is not a directory",
            module_path.as_ref().file_name().unwrap_or_default(),
        );
        parse_objects = false;
    }

    if !graphics_path.exists() {
        warn!(
            "Ignoring graphics directory in {:?} because it does not exist",
            module_path.as_ref().file_name().unwrap_or_default(),
        );
        parse_graphics = false;
    }

    if parse_graphics && !graphics_path.is_dir() {
        warn!(
            "Ignoring graphics directory in {:?} because it is not a directory",
            module_path.as_ref().file_name().unwrap_or_default(),
        );
        parse_graphics = false;
    }

    // Guard against having nothing to parse.
    if !parse_graphics && !parse_objects {
        return Ok(Vec::new());
    }

    let mut results: Vec<Box<dyn RawObject>> = Vec::new();

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
                    results.extend(parser::parse_raw_file(&file_path, options)?);
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
                    results.extend(parser::parse_raw_file(&file_path, options)?);
                }
            }
        }
    }

    Ok(results)
}

/// The function `build_search_string` takes a `raw_object` that implements the `Searchable` trait and
/// returns a string representation of the object for searching purposes.
///
/// Arguments:
///
/// * `raw_object`: The `raw_object` parameter is a reference to an object that implements the
/// `Searchable` trait.
///
/// Returns:
///
/// The function `build_search_string` returns a `String` value.
pub fn build_search_string(raw_object: &dyn Searchable) -> String {
    crate::parser::get_search_string(raw_object)
}
