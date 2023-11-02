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

use options::{ParserOptions, ParsingJob};
use parser::{module_info_file::ModuleInfoFile, raws::RawObject, searchable::Searchable};
use std::path::{Path, PathBuf};
use util::options_has_valid_paths;
use walkdir::DirEntry;

use crate::parser::raw_locations::RawModuleLocation;

pub mod options;
pub mod parser;
#[cfg(feature = "tauri")]
mod tauri_lib;
pub mod util;

#[cfg(feature = "tauri")]
pub use crate::tauri_lib::structs::ProgressPayload;

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
pub fn parse(options: &ParserOptions) -> Vec<Box<dyn RawObject>> {
    parser::parse::parse(options, None)
}

/// Parses the module info file using the provided parser options.
///
/// The only part of the parser options that is used is the `target_path` field and the `job` field.
///
/// Note: This expects the `job` field to be `ParsingJob::SingleModuleInfoFile`. If it is not, it will return an empty `ModuleInfoFile`.
///
/// # Arguments
///
/// * `options` - A reference to the parser options.
///
/// # Returns
///
/// Returns a `ModuleInfoFile` struct containing the parsed module information.
pub fn parse_module_info_file(options: &ParserOptions) -> ModuleInfoFile {
    // Guard against invalid path
    if !options_has_valid_paths(options) {
        log::error!(
            "Returning early for bad path. Provided options:\n{:#?}",
            options
        );
        return ModuleInfoFile::default();
    }
    let target_path = Path::new(&options.target_path);

    if let ParsingJob::SingleModuleInfoFile = options.job {
        // The provided path should be the info.txt file for a module
        parse_module_info_file_direct(&target_path)
    } else {
        log::error!(
            "Wrong job provided to parse module info file! Provided options:\n{:#?}",
            options
        );
        ModuleInfoFile::default()
    }
}

/// Parses the input data to JSON format based on the provided options.
///
/// # Arguments
///
/// * `options` - A reference to the parser options.
///
/// # Returns
///
/// A vector of strings, where each string represents a JSON object.
pub fn parse_to_json(options: &ParserOptions) -> Vec<String> {
    let results = parse(options);
    let mut json_results = Vec::new();
    for result in results {
        json_results.push(serde_json::to_string(&result).unwrap_or_default());
    }
    json_results
}

/// Parses the input data and writes the output to a JSON file, based on the provided options.
///
/// # Arguments
///
/// * `options` - A reference to a `ParserOptions` struct that contains the parsing options.
pub fn parse_to_file(options: &ParserOptions) {
    // Guard against bad output path
    if !options_has_valid_paths(options) {
        log::error!(
            "Returning early for bad output path. Provided options:\n{:#?}",
            options
        );
        return;
    }

    let results = parse_to_json(options);

    util::write_json_string_vec_to_file(&results, &options.output_path);
}

#[cfg(feature = "tauri")]
/// Parse a directory of raws, and return a JSON string of the parsed raws. While parsing, this will
/// emit tauri events to the supplied window. The event is titled `PROGRESS` and it uses the `ProgressPayload`
/// payload for the payload.
///
/// Set the `options` appropriately for the job you want to perform.
///
/// The payload supplies the current progress as a float and the name of the current folder being parsed.
///
/// Properties:
///
/// * `options`: The `ParserOptions` to use for parsing.
/// * `window`: A `tauri::Window` to emit `PROGRESS` events to.
///
/// Returns:
///
/// A JSON string with details on all raws in the game path.
pub fn parse_with_tauri_emit(
    options: &ParserOptions,
    window: tauri::Window,
) -> Vec<Box<dyn RawObject>> {
    // setup progress helper
    let progress_helper = tauri_lib::structs::ProgressHelper::with_tauri_window(window);

    parser::parse::parse(options, Some(&progress_helper))
}

#[cfg(feature = "tauri")]
/// The function `parse_with_tauri_emit_to_json_vec` takes in `options` and `window` as parameters and
/// returns a vector of strings.
///
/// Arguments:
///
/// * `options`: A reference to a `ParserOptions` struct, which contains options for parsing.
/// * `window`: The `window` parameter is of type `tauri::Window`. It represents the window object of
/// the Tauri application.
///
/// Returns:
///
/// A vector of strings is being returned.
pub fn parse_with_tauri_emit_to_json_vec(
    options: &ParserOptions,
    window: tauri::Window,
) -> Vec<String> {
    // setup progress helper
    let progress_helper = tauri_lib::structs::ProgressHelper::with_tauri_window(window);

    let results = parser::parse::parse(options, Some(&progress_helper));

    let mut json_results = Vec::new();
    for result in results {
        json_results.push(serde_json::to_string(&result).unwrap_or_default());
    }
    json_results
}

/// The function `parse_module_info_files_at_location` takes a location path as input, retrieves a list
/// of subdirectories at that location, and parses each subdirectory's "info.txt" file into a
/// `ModuleInfoFile` struct, returning a vector of these structs.
///
/// Arguments:
///
/// * `location_path`: The `location_path` parameter is the path to the directory where the module info
/// files are located. It can be any type that can be converted to a `Path`, such as a `String` or
/// `&str`.
///
/// Returns:
///
/// The function `parse_module_info_files_at_location` returns a vector of `ModuleInfoFile` objects.
fn parse_module_info_files_at_location<P: AsRef<Path>>(location_path: &P) -> Vec<ModuleInfoFile> {
    let mut results: Vec<ModuleInfoFile> = Vec::new();
    let location_path: PathBuf = location_path.as_ref().to_path_buf();
    // Get a list of all subdirectories in the location
    let raw_modules_in_location: Vec<DirEntry> =
        util::subdirectories(location_path.clone()).unwrap_or_default();

    log::info!(
        "Found {} raw modules in {:?}",
        raw_modules_in_location.len(),
        location_path.file_name().unwrap_or_default(),
    );

    // Loop over each module and parse it
    for raw_module in raw_modules_in_location {
        let module_info_file_path = raw_module.path().join("info.txt");
        let module_info_file = parse_module_info_file_direct(&module_info_file_path);
        results.push(module_info_file);
    }

    results
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
fn parse_module_info_file_direct<P: AsRef<Path>>(module_info_file_path: &P) -> ModuleInfoFile {
    // Get information from the module info file
    parser::parse_info_file_from_file_path(module_info_file_path)
}

/// The function `parse_info_modules` parses module information files based on the provided options.
///
/// Arguments:
///
/// * `options`: A reference to a `ParserOptions` struct, which contains various options for parsing
/// module information.
///
/// Returns:
///
/// The function `parse_info_modules` returns a `Vec<ModuleInfoFile>`.
pub fn parse_info_modules(options: &ParserOptions) -> Vec<ModuleInfoFile> {
    // Guard against invalid path
    if !options_has_valid_paths(options) {
        log::error!(
            "draw_json_parser: returning early for bad path. Provided options:\n{:#?}",
            options
        );
        return Vec::new();
    }
    let target_path = Path::new(&options.target_path);
    let mut results: Vec<ModuleInfoFile> = Vec::new();

    match options.job {
        ParsingJob::AllModulesInLocations | ParsingJob::AllModuleInfoFiles => {
            // Set file paths for each location
            let data_path = target_path.join("data");
            let vanilla_path = data_path.join("vanilla");
            let installed_mods_path = data_path.join("installed_mods");
            let workshop_mods_path = target_path.join("mods");

            // Parse each location
            if options
                .locations_to_parse
                .contains(&RawModuleLocation::Vanilla)
            {
                results.extend(parse_module_info_files_at_location(&vanilla_path));
            }
            if options
                .locations_to_parse
                .contains(&RawModuleLocation::InstalledMods)
            {
                results.extend(parse_module_info_files_at_location(&installed_mods_path));
            }
            if options
                .locations_to_parse
                .contains(&RawModuleLocation::Mods)
            {
                results.extend(parse_module_info_files_at_location(&workshop_mods_path));
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
                        "draw_json_parser: No info.txt as expected in {:?}. Is this DF 50.xx? Provided options:\n{:#?}",
                        target_path.file_name().unwrap_or_default(),
                        options
                    );
                }

                return Vec::new();
            }

            let module_info_file = parse_module_info_file_direct(&info_txt_path);
            results.push(module_info_file);
        }
        ParsingJob::SingleRaw => {
            // The provided path should be a raw file directly
            log::warn!(
                "draw_json_parser: Unable to parse info.txt file in this dispatch. Provided options:\n{:#?}",
                options
            );
            return Vec::new();
        }
        ParsingJob::SingleModuleInfoFile => {
            // The provided path should be the info.txt file for a module
            results.push(parse_module_info_file_direct(&target_path));
        }
    }
    log::info!("draw_json_parser: Parsed {} info.txt files", results.len());
    results
}

/// The function `parse_info_modules_to_json` takes a `ParserOptions` object as input, parses the
/// information modules using the options, and returns a vector of JSON strings representing the parsed
/// results.
///
/// Arguments:
///
/// * `options`: A reference to a `ParserOptions` struct.
///
/// Returns:
///
/// The function `parse_info_modules_to_json` returns a vector of strings.
pub fn parse_info_modules_to_json(options: &ParserOptions) -> Vec<String> {
    let results = parse_info_modules(options);
    let mut json_results = Vec::new();
    for result in results {
        json_results.push(serde_json::to_string(&result).unwrap_or_default());
    }
    json_results
}

/// The function `parse_info_modules_to_file` parses information modules to JSON and writes the results
/// to a file.
///
/// Arguments:
///
/// * `options`: A reference to a `ParserOptions` struct.
pub fn parse_info_modules_to_file(options: &ParserOptions) {
    // Guard against bad output path
    if !options_has_valid_paths(options) {
        log::error!(
            "draw_json_parser: Returning early for bad output path. Provided options:\n{:#?}",
            options
        );
        return;
    }

    let results = parse_info_modules_to_json(options);
    util::write_json_string_vec_to_file(&results, &options.output_path);
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
    crate::parser::searchable::get_search_string(raw_object)
}
