//! Utility functions that are used throughout the parser.
//!
//! Include some convenience functions for working with files and directories, as well as functions
//! for validating the provided `ParserOptions` struct.

use std::{
    collections::HashMap,
    fs::File,
    hash::BuildHasher,
    io::{BufWriter, Write},
    path::{Path, PathBuf},
};

use tracing::{debug, error, info, warn};
use walkdir::WalkDir;

use crate::{
    metadata::{ObjectType, ParserOptions, RawObject},
    ParserError,
};

#[tracing::instrument]
/// Get a vec of subdirectories for a given directory
///
/// Using the `WalkDir` crate:
/// 1. create a new `WalkDir` for `directory`
/// 2. limit to immediate contents (`max_depth` and `min_depth` at 1)
/// 3. as an iterator..
///     4. `filter_map` into only non-error results
///     5. filter into only directories
/// 4. collect as a vec
///
/// Arguments:
///
/// * `directory`: The directory to search in
///
/// Returns:
///
/// A vector of all subdirectories as `walkdir::DirEntry`
pub fn subdirectories(directory: PathBuf) -> Result<Vec<walkdir::DirEntry>, ParserError> {
    let root_directory = match directory.canonicalize() {
        Ok(p) => p,
        Err(e) => {
            error!(
                "subdirectories: Unable to canonicalize directory {:?} \n{:?}",
                directory, e
            );
            return Err(ParserError::Io { source: e });
        }
    };

    Ok(WalkDir::new(root_directory)
        .max_depth(1)
        .min_depth(1)
        .into_iter()
        .filter_map(|e| match e {
            Ok(entry) => {
                if entry.path().is_dir() {
                    Some(entry)
                } else {
                    debug!("subdirectories: Entry is not a directory {:?}", entry);
                    None
                }
            }
            Err(e) => {
                error!("subdirectories: Unable to read directory entry \n{:?}", e);
                None
            }
        })
        .collect())
}

/// If the parent directory of the given path exists, return the name of the parent directory, otherwise
/// return "!Unavailable!"
///
/// Arguments:
///
/// * `full_path`: The full path of the file.
///
/// Returns:
///
/// A String
pub fn get_parent_dir_name<P: AsRef<Path>>(full_path: &P) -> String {
    full_path.as_ref().parent().map_or_else(
        || String::from("!Unavailable!"),
        |parent_dir| String::from(parent_dir.file_name().unwrap_or_default().to_string_lossy()),
    )
}

/// "Given a path to a game directory, return a `PathBuf` to that directory if it exists and is a
/// directory, otherwise return an error."
///
/// The first thing we do is create a `PathBuf` from the provided `game_path`. We then check if the path
/// exists and is a directory. If it doesn't exist, we return an error. If it does exist, but isn't a
/// directory, we return an error. If it exists and is a directory, we return the `PathBuf`
///
/// Arguments:
///
/// * `game_path`: &str
///
/// Returns:
///
/// `Result<PathBuf, String>`
///
/// # Errors
///
/// * If the path doesn't exist
pub fn path_from_game_directory<P: AsRef<Path>>(game_path: &P) -> Result<PathBuf, String> {
    //1. "validate" folder is as expected
    let game_path = Path::new(game_path.as_ref());

    // Guard against invalid path
    if !game_path.exists() {
        return Err(String::from(
            "Provided game path for parsing doesn't exist!",
        ));
    }
    if !game_path.is_dir() {
        return Err(String::from("Game path needs to be a directory"));
    }

    // warn on no gamelog.txt
    if !game_path.join("gamelog.txt").exists() {
        warn!("Unable to find gamelog.txt in game directory. Is it valid?");
    }

    Ok(game_path.to_path_buf())
}

/// Save a vector of strings to a file, one string per line.
///
/// Arguments:
///
/// * `parsed_raws_string_vec`: String
/// * `out_filepath`: Path
pub fn write_json_string_vec_to_file<P: AsRef<Path>>(strings_vec: &[String], out_filepath: &P) {
    info!(
        "write_json_string_vec_to_file: Writing {} strings to file {:?}",
        strings_vec.len(),
        out_filepath.as_ref().display()
    );

    if strings_vec.is_empty() {
        warn!("write_json_string_vec_to_file: Provided string vector is empty!");
        return;
    }

    let out_file = match File::create(out_filepath) {
        Ok(f) => f,
        Err(e) => {
            error!(
                "write_json_string_vec_to_file: Unable to open {} for writing \n{:?}",
                out_filepath.as_ref().display(),
                e
            );
            return;
        }
    };

    let mut stream = BufWriter::new(out_file);
    let write_error = &format!(
        "write_json_string_vec_to_file: Unable to write to {}",
        out_filepath.as_ref().to_string_lossy()
    );

    if strings_vec.len() == 1 {
        match writeln!(stream, "{}", strings_vec.first().unwrap_or(&String::new())) {
            Ok(_x) => (),
            Err(e) => {
                error!("write_json_string_vec_to_file: {}\n{:?}", write_error, e);
                return;
            }
        };
        match stream.flush() {
            Ok(_x) => (),
            Err(e) => {
                error!("write_json_string_vec_to_file: {}\n{:?}", write_error, e);
            }
        };
        return;
    }

    let strings_vec = strings_vec.iter();
    // Write the first value with an open bracket '[' at the beginning
    // Write all next values with a comma ',' in front
    // Finish with a closing bracket ']'
    for (i, string) in strings_vec.enumerate() {
        match i {
            0 => match write!(stream, "[{string}") {
                Ok(_x) => (),
                Err(e) => {
                    error!("write_json_string_vec_to_file: {}\n{:?}", write_error, e);
                    return;
                }
            },
            _ => match write!(stream, ",{string}") {
                Ok(_x) => (),
                Err(e) => {
                    error!("write_json_string_vec_to_file: {}\n{:?}", write_error, e);
                    return;
                }
            },
        }
    }

    match writeln!(stream, "]") {
        Ok(_x) => (),
        Err(e) => {
            error!("write_json_string_vec_to_file: {}\n{:?}", write_error, e);
            return;
        }
    };

    match stream.flush() {
        Ok(_x) => (),
        Err(e) => {
            error!("write_json_string_vec_to_file: {}\n{:?}", write_error, e);
        }
    };
}

#[allow(clippy::too_many_lines)]
#[tracing::instrument]
/// The function `validate_options` validates the provided `ParserOptions` struct.
///
/// It checks that the provided paths exist and are valid. It will also expand any relative
/// paths to absolute paths.
///
/// It returns a `ParserOptions` struct if all paths are valid, otherwise it returns `None`.
///
/// Arguments:
///
/// * `options`: The `ParserOptions` struct to validate.
///
/// Returns:
///
/// An `Option<ParserOptions>` struct. None if options were invalid.
pub fn validate_options(options: &ParserOptions) -> Result<ParserOptions, ParserError> {
    // Copy the options into a new struct, before we validate the paths
    let mut validated_options = ParserOptions {
        attach_metadata_to_raws: options.attach_metadata_to_raws,
        locations_to_parse: options.locations_to_parse.clone(),
        object_types_to_parse: options.object_types_to_parse.clone(),
        skip_apply_copy_tags_from: options.skip_apply_copy_tags_from,
        skip_apply_creature_variations: options.skip_apply_creature_variations,
        ..Default::default()
    };

    // Guard against invalid path if locations are set
    if !options.locations_to_parse.is_empty() {
        let target_path = &options.dwarf_fortress_directory;

        // Canonicalize the path
        let target_path = match target_path.canonicalize() {
            Ok(p) => p,
            Err(e) => {
                return Err(ParserError::InvalidOptions(format!(
                    "Unable to canonicalize Dwarf Fortress path!\n{target_path:?}\n{e:?}"
                )))
            }
        };

        if !target_path.exists() {
            return Err(ParserError::InvalidOptions(format!(
                "Provided Dwarf Fortress path for doesn't exist!\n{}",
                target_path.display()
            )));
        }

        if !target_path.is_dir() {
            return Err(ParserError::InvalidOptions(format!(
                "Dwarf Fortress path needs to be a directory!\n{}",
                target_path.display()
            )));
        }

        validated_options
            .dwarf_fortress_directory
            .clone_from(&target_path);
    }

    // Validate any raw file paths
    for raw_file_path in &options.raw_files_to_parse {
        if !raw_file_path.exists() {
            warn!(
                "options_validator: Discarding non-existent raw file:\n{}",
                raw_file_path.display()
            );
        } else if !raw_file_path.is_file() {
            warn!(
                "options_validator: Discarding raw file because it isn't a file:\n{}",
                raw_file_path.display()
            );
        } else {
            // Add the canonicalized path to the raw file
            let raw_file_path = raw_file_path.canonicalize().unwrap_or_else(|e| {
                warn!(
                    "options_validator: Discarding raw file that cannot be canonicalized:\n{:?}",
                    e
                );
                raw_file_path.clone()
            });
            validated_options.raw_files_to_parse.push(raw_file_path);
        }
    }

    // Validate any raw module paths
    for raw_module_path in &options.raw_modules_to_parse {
        if !raw_module_path.exists() {
            warn!(
                "options_validator: Discarding non-existent raw module directory:\n{}",
                raw_module_path.display()
            );
        } else if !raw_module_path.is_dir() {
            warn!(
              "options_validator: Discarding raw module directory because it isn't a directory:\n{}",
              raw_module_path.display()
          );
        } else {
            // Add the canonicalized path to the module
            let raw_module_path = raw_module_path.canonicalize().unwrap_or_else(|e| {
              warn!(
                  "options_validator: Discarding raw module directory path that cannot be canonicalized:\n{:?}",
                  e
              );
              raw_module_path.clone()
          });
            validated_options.raw_modules_to_parse.push(raw_module_path);
        }
    }

    // Validate any legends export paths
    for legends_export_path in &options.legends_exports_to_parse {
        if !legends_export_path.exists() {
            warn!(
                "options_validator: Discarding non-existent legends export:\n{}",
                legends_export_path.display()
            );
        } else if !legends_export_path.is_file() {
            warn!(
                "options_validator: Discarding legends export because it isn't a file:\n{}",
                legends_export_path.display()
            );
        } else {
            // Add the canonicalized path to the legends export
            let legends_export_path = legends_export_path.canonicalize().unwrap_or_else(|e| {
              warn!(
                  "options_validator: Discarding legends export path that cannot be canonicalized\n{:?}",
                  e
              );
              legends_export_path.clone()
          });
            validated_options
                .legends_exports_to_parse
                .push(legends_export_path);
        }
    }

    // Validate any module info file paths
    for module_info_file_path in &options.module_info_files_to_parse {
        if !module_info_file_path.exists() {
            warn!(
                "options_validator: Discarding non-existent module info file:\n{}",
                module_info_file_path.display()
            );
        } else if !module_info_file_path.is_file() {
            warn!(
                "options_validator: Discarding module info file because it isn't a file:\n{}",
                module_info_file_path.display()
            );
        } else {
            // Add the canonicalized path to the module info file
            let module_info_file_path = module_info_file_path.canonicalize().unwrap_or_else(|e| {
              warn!(
                  "options_validator: Discarding module info file path that cannot be canonicalized\n{:?}",
                  e
              );
              module_info_file_path.clone()
          });
            validated_options
                .module_info_files_to_parse
                .push(module_info_file_path);
        }
    }

    Ok(validated_options)
}

/// The function `get_only_creatures_from_raws` takes a slice of `RawObject` trait objects and returns a
/// vector containing only the objects that are of type `DFCreature`.
///
/// Arguments:
///
/// * `all_raws`: A slice of boxed objects that implement the `RawObject` trait.
///
/// Returns:
///
/// a vector of `DFCreature` objects.
pub fn get_only_creatures_from_raws(all_raws: &[Box<dyn RawObject>]) -> Vec<Creature> {
    all_raws
        .iter()
        .filter(|r| r.get_type() == &ObjectType::Creature)
        .map(|r| r.as_any().downcast_ref::<Creature>())
        .map(|r| r.unwrap_or(&Creature::default()).clone())
        .collect::<Vec<Creature>>()
}

/// The function `get_only_select_creatures_from_raws` filters a slice of raw objects and returns a
/// vector containing only the objects of type `SelectCreature`.
///
/// Arguments:
///
/// * `all_raws`: A slice of boxed objects that implement the `RawObject` trait.
///
/// Returns:
///
/// a vector of `SelectCreature` objects.
pub fn get_only_select_creatures_from_raws(all_raws: &[Box<dyn RawObject>]) -> Vec<SelectCreature> {
    all_raws
        .iter()
        .filter(|r| r.get_type() == &ObjectType::SelectCreature)
        .map(|r| r.as_any().downcast_ref::<SelectCreature>())
        .map(|r| r.unwrap_or(&SelectCreature::default()).clone())
        .collect::<Vec<SelectCreature>>()
}

/// `try_get_file` attempts to open a file at the given path and returns a `File` if successful.
///
/// Arguments:
///
/// * `file_path`: A path to the file to open.
///
/// Returns:
///
/// An `Option<File>`. None if the file doesn't exist or isn't a file.
///
/// # Errors
///
/// * `ParserError::Io` if the file cannot be opened or doesn't exist
pub fn try_get_file<P: AsRef<Path>>(file_path: &P) -> Result<File, ParserError> {
    // Validate file exists
    if !file_path.as_ref().exists() {
        debug!(
            "try_get_file: Path doesn't exist:\n{}",
            file_path.as_ref().display()
        );
        return Err(ParserError::Io {
            source: std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!(
                    "try_get_file: Path doesn't exist {}",
                    file_path.as_ref().display()
                ),
            ),
        });
    }
    if !file_path.as_ref().is_file() {
        debug!(
            "try_get_file: Path does not point to a file:\n{}",
            file_path.as_ref().display(),
        );
        return Err(ParserError::Io {
            source: std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!(
                    "try_get_file: Path does not point to a file {}",
                    file_path.as_ref().display()
                ),
            ),
        });
    }

    // Open the file
    match File::open(file_path) {
        Ok(file) => Ok(file),
        Err(e) => {
            debug!(
                "try_get_file: Unable to open file {}\n{:?}",
                file_path.as_ref().display(),
                e
            );
            Err(ParserError::Io { source: e })
        }
    }
}

/// Create a summary of the parsed raws.
///
/// Summarizes the parsed raws by object type.
///
/// Arguments:
///
/// * `raws`: A slice of boxed objects that implement the `RawObject` trait.
///
/// Returns:
///
/// A `HashMap<ObjectType, usize>` where the key is the object type and the value is the number of
/// objects of that type.
pub fn summarize_raws(raws: &[Box<dyn RawObject>]) -> HashMap<ObjectType, usize> {
    let mut summary: std::collections::HashMap<ObjectType, usize> =
        std::collections::HashMap::new();

    for raw in raws {
        let count = summary.entry(raw.get_type().clone()).or_insert(0);
        *count += 1;
    }

    summary
}

/// Logs a summary of the parsed raws to the console via `tracing::info!`
///
/// Arguments:
///
/// * `summary`: A `HashMap<ObjectType, usize>` where the key is the object type and the value is the number of
/// objects of that type.
pub fn log_summary<S: BuildHasher>(summary: &HashMap<ObjectType, usize, S>) {
    let total = summary.values().sum::<usize>();

    info!("Summary of parsed raws:");
    for (object_type, count) in summary {
        info!("\t{count}\t{object_type}");
    }
    info!("Total: {total}");
}

/// The function `build_object_id_from_pieces` takes in metadata, an identifier, and a raw type, and
/// returns a formatted string. This is a convenience function for building object IDs so they all
/// follow the same format.
///
/// Arguments:
///
/// * `metadata`: A reference to a `RawMetadata` object.
/// * `identifier`: A string representing the identifier of the object.
/// * `raw_type`: The `raw_type` parameter is of type `ObjectType`.
///
/// Returns:
///
/// a string.
#[must_use]
pub fn build_object_id_from_pieces(
    metadata: &RawMetadata,
    identifier: &str,
    raw_type: &ObjectType,
) -> String {
    format!(
        "{raw_parent_id}-{raw_type}-{raw_id}-{module_name}{module_version}",
        raw_id = slugify(identifier),
        raw_parent_id = slugify(metadata.get_raw_identifier()),
        module_version = metadata.get_module_numerical_version(),
        module_name = slugify(metadata.get_module_name()),
    )
}

/// It takes a slice of strings, parses the first two strings as unsigned 16-bit integers, and returns a
/// two-element array of unsigned 16-bit integers
///
/// Arguments:
///
/// * `split`: &[&str] - This is the array of strings that we're going to parse.
///
/// Returns:
///
/// A Result<[u16; 2], `ParseIntError`>
///
/// # Errors
///
/// Will return a `ParseIntError` if the string cannot be parsed into two unsigned 16-bit integers.
pub fn parse_min_max_range_from_vec(split: &[&str]) -> Result<[u32; 2], ParseIntError> {
    let min: u32 = match split.first().unwrap_or(&"").parse() {
        Ok(n) => n,
        Err(e) => {
            error!("min_value parsing error\n{:?}", e);
            return Err(e);
        }
    };
    let max: u32 = match split.get(1).unwrap_or(&"").parse() {
        Ok(n) => n,
        Err(e) => {
            error!("max_value parsing error\n{:?}", e);
            return Err(e);
        }
    };
    Ok([min, max])
}

/// The function `parse_min_max_range` takes a string input and splits it by a colon, then calls another
/// function to parse the resulting vector into an array of two unsigned 16-bit integers.
///
/// Arguments:
///
/// * `value`: A string representing a range of values in the format "min:max".
///
/// Returns:
///
/// The function `parse_min_max_range` returns a `Result<[u16; 2], ParseIntError>`.
///
/// # Errors
///
/// Will return a `ParseIntError` if the string cannot be parsed into two unsigned 16-bit integers.
pub fn parse_min_max_range(value: &str) -> Result<[u32; 2], ParseIntError> {
    let split: Vec<&str> = value.split(':').collect::<Vec<&str>>();
    parse_min_max_range_from_vec(&split)
}

/// The function `clone_raw_vector_with_purge` clones a vector of raw objects, excluding those with
/// specified object IDs to purge.
///
/// Arguments:
///
/// * `all_raws`: A slice of `Box<dyn RawObject>`, which represents a collection of raw objects.
/// * `object_ids_to_purge`: A slice of string references representing the object IDs that need to be
/// purged from the `all_raws` vector.
///
/// Returns:
///
/// The function `clone_raw_vector_with_purge` returns a vector of boxed dynamic objects (`Vec<Box<dyn
/// RawObject>>`).
pub fn with_purge(
    all_raws: &[Box<dyn RawObject>],
    object_ids_to_purge: &[&str],
) -> Vec<Box<dyn RawObject>> {
    let mut new_raws: Vec<Box<dyn RawObject>> = Vec::new();

    for raw in all_raws {
        if object_ids_to_purge.contains(&raw.get_object_id()) {
            trace!("clone_raw_vector purging {}", raw.get_object_id());
        } else {
            // Match the object type, downcast and clone into a new box in new_raws
            new_raws.push(clone_raw_object_box(raw));
        }
    }
    new_raws
}

#[allow(clippy::borrowed_box)]
/// The function `clone_raw_object_box` clones a boxed object based on its type.
///
/// Arguments:
///
/// * `box_ref`: A reference to a boxed object implementing the `RawObject` trait.
///
/// Returns:
///
/// The function `clone_raw_object_box` returns a `Box<dyn RawObject>`.
pub fn clone_raw_object_box(box_ref: &Box<dyn RawObject>) -> Box<dyn RawObject> {
    match box_ref.get_type() {
        ObjectType::Creature => {
            let temp_creature = box_ref
                .as_any()
                .downcast_ref::<Creature>()
                .unwrap_or(&Creature::empty())
                .clone();
            Box::new(temp_creature)
        }
        ObjectType::SelectCreature => {
            let temp_select_creature = box_ref
                .as_any()
                .downcast_ref::<SelectCreature>()
                .unwrap_or(&SelectCreature::empty())
                .clone();
            Box::new(temp_select_creature)
        }
        ObjectType::CreatureVariation => {
            let temp_creature_variation = box_ref
                .as_any()
                .downcast_ref::<CreatureVariation>()
                .unwrap_or(&CreatureVariation::empty())
                .clone();
            Box::new(temp_creature_variation)
        }
        ObjectType::Plant => {
            let temp_plant = box_ref
                .as_any()
                .downcast_ref::<Plant>()
                .unwrap_or(&Plant::empty())
                .clone();
            Box::new(temp_plant)
        }
        ObjectType::Inorganic => {
            let temp_inorganic = box_ref
                .as_any()
                .downcast_ref::<Inorganic>()
                .unwrap_or(&Inorganic::empty())
                .clone();
            Box::new(temp_inorganic)
        }
        ObjectType::MaterialTemplate => {
            let temp_material_template = box_ref
                .as_any()
                .downcast_ref::<MaterialTemplate>()
                .unwrap_or(&MaterialTemplate::empty())
                .clone();
            Box::new(temp_material_template)
        }
        ObjectType::Graphics => {
            let temp_graphic = box_ref
                .as_any()
                .downcast_ref::<Graphic>()
                .unwrap_or(&Graphic::empty())
                .clone();
            Box::new(temp_graphic)
        }
        ObjectType::TilePage => {
            let temp_tile_page = box_ref
                .as_any()
                .downcast_ref::<TilePage>()
                .unwrap_or(&TilePage::empty())
                .clone();
            Box::new(temp_tile_page)
        }
        ObjectType::Entity => {
            let temp_entity = box_ref
                .as_any()
                .downcast_ref::<Entity>()
                .unwrap_or(&Entity::empty())
                .clone();
            Box::new(temp_entity)
        }
        _ => {
            warn!(
                "clone_raw_object_box has an unhandled object type: {:?}",
                box_ref.get_type()
            );
            Box::new(Creature::empty())
        }
    }
}

/// The function `with_limit_and_page` takes a slice of `RawObject` objects, a limit, and a page number,
/// and returns a new vector containing a subset of the original objects based on the limit and page
/// number.
///
/// Arguments:
///
/// * `all_raws`: A slice of boxed objects that implement the `RawObject` trait.
/// * `limit`: The `limit` parameter specifies the maximum number of items to be included in each page
/// of the result.
/// * `page`: The `page` parameter represents the page number of the data you want to retrieve. It is
/// used to calculate the starting and ending positions of the data based on the `limit` parameter. The
/// first page is represented by page number 1, so if you want to retrieve data from the first page
///
/// Returns:
///
/// a vector of boxed dynamic objects (`Vec<Box<dyn RawObject>>`).
pub fn with_limit_and_page(
    all_raws: &[Box<dyn RawObject>],
    limit: usize,
    page: usize,
) -> Vec<Box<dyn RawObject>> {
    let mut new_raws: Vec<Box<dyn RawObject>> = Vec::new();
    // Page 0 is the first page, so we need to subtract 1 from the page number
    // But this guards against someone sending an invalid page number of 0
    let page = if page > 0 { page - 1 } else { page };
    let start = limit * page;
    let end = start + limit;

    debug!("with_limit_and_page start: {start}, end: {end}, page: {page}");

    for (pos, raw) in all_raws.iter().enumerate() {
        if pos >= start && pos < end {
            new_raws.push(clone_raw_object_box(raw));
        }
    }
    new_raws
}
