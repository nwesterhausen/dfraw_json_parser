/*!
`dfraw_json_parser` provides a way to turn raw files from Dwarf Fortress into JSON. It's
currently useful for getting some basic information from from a limited set of raw types:

- creatures
- plants
- inorganics (rocks, ores, etc)


## How It Works

It functions using a regular expression to break apart the tokens and then checks the
key in the token against a long list of ones it knows about. Any matches are utilized to
build its own representation of the raw before a final step where it dumps it to JSON and
returns a JSON string.

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

use parser::raws::RawModuleLocation;
use parser::{DFParser, TypedJsonSerializable};
use std::path::{Path, PathBuf};
use walkdir::DirEntry;

mod parser;
#[cfg(feature = "tauri")]
mod tauri_lib;
mod util;

//Todo: add an option to apply any COPY_TAG_FROM and APPLY_CREATURE_VARIATION tags
//Todo: add an option to specify what kinds of raws to parse
//Todo: add a parameter for JSON_FILE name (currently hard-coded to out.json)

/// Parse all raws within the module location to JSON.
///
/// The actual structure of the array returned is `<T extends Raw>[][][]`, where T can be `Creature`,
/// `Inorganic`, or `Plant`. (See `typings.d.ts`)
///
/// The three levels of arrays are:
///
/// 1. Raw Modules (within this location)
/// 2. Raw Files (within each module)
/// 3. Raws (within each raw file)
///
/// Arguments:
///
/// * `raw_module_location_path`: The path to the directory containing the raw modules. This has subdirectories
/// which contain an 'info.txt' file and raw objects or graphics.
///
/// Returns:
///
/// A JSON string: `<T extends Raw>[][][]`, where T can be `Creature`, `Inorganic`, or `Plant`.
/// (See [`typings.d.ts`](https://github.com/nwesterhausen/dfraw_json_parser/blob/main/typing.d.ts))
pub fn parse_module_location<P: AsRef<Path>>(raw_module_location: &P) -> String {
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
    let module_location = RawModuleLocation::from_sourced_directory(
        raw_module_location_path
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default(),
    );

    //3. Get list of all subdirectories
    let raw_module_iter: Vec<DirEntry> =
        util::subdirectories(PathBuf::from(raw_module_location_path)).unwrap_or_default();

    log::info!(
        "{num} raw modules located in {location:?}",
        num = raw_module_iter.len(),
        location = module_location
    );

    let mut all_json: Vec<String> = Vec::new();
    //4. Loop over all raw modules in the raw module directory
    for raw_module_directory in raw_module_iter {
        //2. Parse raws and dump JSON into array
        all_json.push(parse_raw_module(&raw_module_directory.path()));
    }

    format!("[{}]", all_json.join(","))
}

/// Parse all info.txt files within the module location to JSON.
///
/// This would be useful to get information *about* the raw modules in the specified location without reading
/// and parsing all the raws.
///
/// Arguments:
///
/// * `raw_module_location_path`: The path to the directory containing the raw modules. This has subdirectories
/// which contain an 'info.txt' file and raw objects or graphics.
///
/// Returns:
///
/// A JSON string: `InfoFile[]`
/// (See [`typings.d.ts`](https://github.com/nwesterhausen/dfraw_json_parser/blob/main/typing.d.ts))
pub fn parse_info_txt_in_location<P: AsRef<Path>>(raw_module_location: &P) -> String {
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
    let module_location = RawModuleLocation::from_sourced_directory(
        raw_module_location_path
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default(),
    );

    //3. Get list of all subdirectories
    let raw_module_iter: Vec<DirEntry> =
        util::subdirectories(PathBuf::from(raw_module_location_path)).unwrap_or_default();

    log::info!(
        "{num} raw modules located in {location:?}",
        num = raw_module_iter.len(),
        location = module_location
    );

    let mut all_json: Vec<String> = Vec::new();
    //4. Loop over all raw modules in the raw module directory
    for raw_module_directory in raw_module_iter {
        //2. Parse raws and dump JSON into array
        all_json.push(parse_info_txt_in_module(&raw_module_directory.path()));
    }

    format!("[{}]", all_json.join(","))
}

/// Parse all raw files within the game directory to JSON.
///
/// Arguments:
///
/// * `df_game_path`: The path to the game directory. This is the directory that contains the data,
/// mods, and gamelog.txt files.
///
/// Returns:
///
/// A JSON string: `<T extends Raw>[][][][]`, where T can be `Creature`, `Inorganic`, or `Plant`.
/// (See [`typings.d.ts`](https://github.com/nwesterhausen/dfraw_json_parser/blob/main/typing.d.ts))
pub fn parse_game_raws<P: AsRef<Path>>(df_game_path: &P) -> String {
    //1. "validate" folder is as expected
    let game_path = Path::new(df_game_path.as_ref());
    // Guard against invalid path
    if !game_path.exists() {
        log::error!(
            "Provided game path for parsing doesn't exist!\n{}",
            game_path.display()
        );
        return String::new();
    }
    if !game_path.is_dir() {
        log::error!("Game path needs to be a directory {}", game_path.display());
        return String::new();
    }

    // warn on no gamelog.txt
    if !game_path.join("gamelog.txt").exists() {
        log::warn!("Unable to find gamelog.txt in game directory. Is it valid?");
    }

    // Set file paths for vanilla raw modules, workshop mods and installed mods
    let data_path = game_path.join("data");
    let vanilla_path = data_path.join("vanilla");
    let installed_mods_path = data_path.join("installed_mods");
    let workshop_mods_path = game_path.join("mods");

    let all_json = vec![
        parse_module_location(&vanilla_path),
        parse_module_location(&installed_mods_path),
        parse_module_location(&workshop_mods_path),
    ];

    let non_empty_json: Vec<String> = all_json
        .into_iter()
        .filter(|s| !String::is_empty(s))
        .collect();

    format!("[{}]", non_empty_json.join(","))
}

/// Parse a raw module to JSON.
///
/// Arguments:
///
/// * `raw_module_path`: The path to the raw module directory.
///
/// Returns:
///
/// A JSON string: `<T extends Raw>[][]]`, where T can be `Creature`, `Inorganic`, or `Plant`.
/// (See [`typings.d.ts`](https://github.com/nwesterhausen/dfraw_json_parser/blob/main/typing.d.ts))
pub fn parse_raw_module<P: AsRef<Path>>(raw_module_path: &P) -> String {
    DFParser::parse_raw_module_to_json_string(raw_module_path)
}

/// Parse all the game raws and saves the result to a JSON file.
///
/// Arguments:
///
/// * `df_game_path`: The path to the Dwarf Fortress install directory.
/// * `out_filepath`: The path to the file to save the parsed raws to. (This should end in `.json`.)
pub fn parse_game_raws_to_file<P: AsRef<Path>>(df_game_path: &P, out_filepath: &Path) {
    let parsed_json_string = parse_game_raws(df_game_path);
    util::write_json_string_to_file(&parsed_json_string, out_filepath);
}

/// Parse all info.txt within the modules found in the game directory and saves a JSON file.
///
/// Arguments:
///
/// * `df_game_path`: The path to the raws folder.
/// * `out_filepath`: The path to the file to save the parsed raws to. (This should end in `.json`.)
pub fn parse_info_txt_in_game_dir_to_file<P: AsRef<Path>>(df_game_path: &P, out_filepath: &Path) {
    let parsed_json_string = parse_info_txt_in_game_dir(df_game_path);
    util::write_json_string_to_file(&parsed_json_string, out_filepath);
}

/// Parse an info.txt file from a raw module to JSON.
///
/// Arguments:
///
/// * `raw_module_directory`: The path of the raw module to parse the info.txt for
///
/// Returns:
///
/// A JSON string: `InfoFile`
/// (See [`typings.d.ts`](https://github.com/nwesterhausen/dfraw_json_parser/blob/main/typing.d.ts))
pub fn parse_info_txt_in_module<P: AsRef<Path>>(raw_module_directory: &P) -> String {
    let dfraw_module_info = DFParser::parse_info_file_from_module_directory(raw_module_directory);
    match dfraw_module_info.to_typed_json_string() {
        Ok(s) => {
            return s;
        }
        Err(e) => {
            log::error!("Failure to serialize parsed raw data\n{}", e);
        }
    }

    String::new()
}

/// Parse a single raw file to JSON.
///
/// Arguments:
///
/// * `raw_file`: The path to the raw file to parse.
///
/// Returns:
///
/// A JSON string: `<T extends Raw>[]`, where T can be `Creature`, `Inorganic`, or `Plant`.
/// (See [`typings.d.ts`](https://github.com/nwesterhausen/dfraw_json_parser/blob/main/typing.d.ts))
pub fn parse_single_raw_file<P: AsRef<Path>>(raw_file: &P) -> String {
    DFParser::parse_single_raw_file_to_json_string(raw_file)
}

/// Parse all info.txt within the modules found in the game directory to JSON.
///
/// Arguments:
///
/// * `df_game_path`: The path to the the DF game directory.
///
/// Returns:
///
/// A JSON string: `InfoFile[][][]`
/// (See [`typings.d.ts`](https://github.com/nwesterhausen/dfraw_json_parser/blob/main/typing.d.ts))
pub fn parse_info_txt_in_game_dir<P: AsRef<Path>>(df_game_path: &P) -> String {
    //1. "validate" folder is as expected
    let game_path = match util::path_from_game_directory(df_game_path) {
        Ok(p) => p,
        Err(e) => {
            log::error!("Game Path Error: {}", e);
            return String::new();
        }
    };

    // Set file paths for vanilla raw modules, workshop mods and installed mods
    let data_path = game_path.join("data");
    let vanilla_path = data_path.join("vanilla");
    let installed_mods_path = data_path.join("installed_mods");
    let workshop_mods_path = game_path.join("mods");

    let all_json = vec![
        parse_info_txt_in_location(&vanilla_path),
        parse_info_txt_in_location(&installed_mods_path),
        parse_info_txt_in_location(&workshop_mods_path),
    ];

    format!("[{}]", all_json.join(","))
}

/// Parse a single raw file and writes the parsed raw array to a JSON file.
///
/// Arguments:
///
/// * `raw_file`: The path to the raw file to read.
/// * `out_filepath`: The path to the file you want to write to.
pub fn parse_single_raw_file_to_file<P: AsRef<Path>>(raw_file: &P, out_filepath: &Path) {
    let parsed_json_string = parse_single_raw_file(raw_file);
    util::write_json_string_to_file(&parsed_json_string, out_filepath);
}

/// Parse a single raw module directory, and writes the parsed JSON string to a file.
///
/// Arguments:
///
/// * `module_path`: The path to the raw file to read.
/// * `out_filepath`: The path to the file you want to write to.
pub fn parse_raw_module_to_file<P: AsRef<Path>>(module_path: &P, out_filepath: &Path) {
    let parsed_json_string = parse_raw_module(module_path);
    util::write_json_string_to_file(&parsed_json_string, out_filepath);
}

/// Parse all the raw modules within a raw module location, and writes the parsed JSON string to a file.
///
/// Arguments:
///
/// * `raw_module_location_path`: The path to the raw module directory.
/// * `out_filepath`: The path to the file you want to write to.
pub fn parse_module_location_to_file<P: AsRef<Path>>(
    raw_module_location_path: &P,
    out_filepath: &Path,
) {
    let parsed_json_string = parse_module_location(raw_module_location_path);
    util::write_json_string_to_file(&parsed_json_string, out_filepath);
}

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
pub fn parse_game_raws_with_tauri_emit<P: AsRef<Path>>(
    df_game_path: &P,
    window: tauri::Window,
) -> String {
    tauri_lib::parse_game_raws_with_tauri_emit(df_game_path, window)
}

#[cfg(feature = "tauri")]
#[allow(clippy::cast_precision_loss)]
/// It takes a path to a directory containing raw modules, Parse them, and returns a JSON string
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
pub fn parse_location_with_tauri_emit<P: AsRef<Path>>(
    location_path: &P,
    window: tauri::Window,
) -> String {
    tauri_lib::parse_location_with_tauri_emit(location_path, window)
}
