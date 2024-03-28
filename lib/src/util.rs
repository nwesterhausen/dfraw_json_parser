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
    parser::{creature::Creature, select_creature::SelectCreature, ObjectType, RawObject},
    ParserError, ParserOptions,
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
    match full_path.as_ref().parent() {
        Some(parent_dir) => {
            String::from(parent_dir.file_name().unwrap_or_default().to_string_lossy())
        }
        None => String::from("!Unavailable!"),
    }
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

/// Save a vector of parsed raw objects to a file in JSON format.
///
/// Arguments:
///
/// * `raws_vec`: A vector of boxed objects that implement the `RawObject` trait.
/// * `out_filepath`: A path to the output file.
/// * `pretty_print`: A boolean value indicating whether to pretty print the JSON output.
pub fn write_raw_vec_to_file<P: AsRef<Path>>(
    raws_vec: &Vec<Box<dyn RawObject>>,
    out_filepath: &P,
    pretty_print: bool,
) {
    info!(
        "write_raw_vec_to_file: Writing {} raws to file {:?}",
        raws_vec.len(),
        out_filepath.as_ref().display()
    );

    if raws_vec.is_empty() {
        warn!("write_raw_vec_to_file: Provided raw vector is empty!");
        return;
    }

    let out_file = match File::create(out_filepath) {
        Ok(f) => f,
        Err(e) => {
            error!(
                "write_raw_vec_to_file: Unable to open {} for writing \n{:?}",
                out_filepath.as_ref().display(),
                e
            );
            return;
        }
    };

    if pretty_print {
        serde_json::to_writer_pretty(out_file, raws_vec).unwrap_or_else(|e| {
            error!(
                "write_raw_vec_to_file: Unable to write to {} \n{:?}",
                out_filepath.as_ref().display(),
                e
            );
        });
    } else {
        serde_json::to_writer(out_file, raws_vec).unwrap_or_else(|e| {
            error!(
                "write_raw_vec_to_file: Unable to write to {} \n{:?}",
                out_filepath.as_ref().display(),
                e
            );
        });
    }
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

        validated_options.dwarf_fortress_directory = target_path.clone();
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

/// The function `raws_to_string` converts a vector of raw objects into a JSON string representation.
///
/// Arguments:
///
/// * `raws`: The `raws` parameter is a vector of `Box<dyn RawObject>`.
///
/// Returns:
///
/// The function `raws_to_string` returns a `String` that represents the input `Vec<Box<dyn RawObject>>`
/// as a JSON array.
pub fn raws_to_string(raws: Vec<Box<dyn RawObject>>) -> String {
    // It should be an array, so start with '[' character,
    // then add each raw object, separated by a comma.
    // Finally add the closing ']' character.
    // (The last item cannot have a comma before ']')
    let mut json = String::from('[');
    for raw in raws {
        json.push_str(serde_json::to_string(&raw).unwrap_or_default().as_str());
        json.push(',');
    }
    json.pop(); // remove trailing comma
    json.push(']');
    json
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
