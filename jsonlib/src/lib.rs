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

use std::path::{Path, PathBuf};
use tracing::{debug, error, info, warn};
use util::validate_options;
use walkdir::{DirEntry, WalkDir};

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
/// - `write_raw_vec_to_file`: Writes a vector of raw objects to a file in JSON format.
/// - `write_json_string_vec_to_file`: Writes a vector of strings to a file, with each string on a separate line.
/// - `raws_to_string`: Converts a vector of raw objects to a JSON string representation.
pub mod util;

#[cfg(feature = "tauri")]
pub use tauri_lib::ProgressDetails;
#[cfg(feature = "tauri")]
pub use tauri_lib::ProgressPayload;
#[cfg(feature = "tauri")]
pub use tauri_lib::ProgressTask;

#[cfg(feature = "tauri")]
#[allow(clippy::too_many_lines)]
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
    use dfraw_parser::{metadata::ParserOptions, ParseResult, ParserError};

    let mut progress_helper = tauri_lib::ProgressHelper::with_tauri_window(window);
    tauri_lib::parse(options, &mut progress_helper)
}
