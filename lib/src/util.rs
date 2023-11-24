use std::{
    fs::File,
    io::{BufWriter, Write},
    path::{Path, PathBuf},
};

use walkdir::WalkDir;

use crate::{
    options::ParserOptions,
    parser::{
        creature::raw::Creature, object_types::ObjectType, raws::RawObject,
        select_creature::raw::SelectCreature,
    },
};

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
pub fn subdirectories(directory: PathBuf) -> Option<Vec<walkdir::DirEntry>> {
    if !(directory.exists() && directory.is_dir()) {
        return None;
    }
    Some(
        WalkDir::new(directory)
            .max_depth(1)
            .min_depth(1)
            .into_iter()
            .filter_map(std::result::Result::ok)
            .filter(|e| e.file_type().is_dir())
            .collect(),
    )
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
/// A Result<PathBuf, String>
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
        log::warn!("Unable to find gamelog.txt in game directory. Is it valid?");
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
    log::info!(
        "write_raw_vec_to_file: Writing {} raws to file {:?}",
        raws_vec.len(),
        out_filepath.as_ref().display()
    );

    if raws_vec.is_empty() {
        log::warn!("write_raw_vec_to_file: Provided raw vector is empty!");
        return;
    }

    let out_file = match File::create(out_filepath) {
        Ok(f) => f,
        Err(e) => {
            log::error!(
                "write_raw_vec_to_file: Unable to open {} for writing \n{:?}",
                out_filepath.as_ref().display(),
                e
            );
            return;
        }
    };

    if pretty_print {
        serde_json::to_writer_pretty(out_file, raws_vec).unwrap_or_else(|e| {
            log::error!(
                "write_raw_vec_to_file: Unable to write to {} \n{:?}",
                out_filepath.as_ref().display(),
                e
            );
        });
    } else {
        serde_json::to_writer(out_file, raws_vec).unwrap_or_else(|e| {
            log::error!(
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
pub fn write_json_string_vec_to_file<P: AsRef<Path>>(strings_vec: &Vec<String>, out_filepath: &P) {
    log::info!(
        "write_json_string_vec_to_file: Writing {} strings to file {:?}",
        strings_vec.len(),
        out_filepath.as_ref().display()
    );

    if strings_vec.is_empty() {
        log::warn!("write_json_string_vec_to_file: Provided string vector is empty!");
        return;
    }

    let out_file = match File::create(out_filepath) {
        Ok(f) => f,
        Err(e) => {
            log::error!(
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
                log::error!("write_json_string_vec_to_file: {}\n{:?}", write_error, e);
                return;
            }
        };
        match stream.flush() {
            Ok(_x) => (),
            Err(e) => {
                log::error!("write_json_string_vec_to_file: {}\n{:?}", write_error, e);
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
                    log::error!("write_json_string_vec_to_file: {}\n{:?}", write_error, e);
                    return;
                }
            },
            _ => match write!(stream, ",{string}") {
                Ok(_x) => (),
                Err(e) => {
                    log::error!("write_json_string_vec_to_file: {}\n{:?}", write_error, e);
                    return;
                }
            },
        }
    }

    match writeln!(stream, "]") {
        Ok(_x) => (),
        Err(e) => {
            log::error!("write_json_string_vec_to_file: {}\n{:?}", write_error, e);
            return;
        }
    };

    match stream.flush() {
        Ok(_x) => (),
        Err(e) => {
            log::error!("write_json_string_vec_to_file: {}\n{:?}", write_error, e);
        }
    };
}

#[allow(clippy::too_many_lines)]
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
pub fn validate_options(options: &ParserOptions) -> Option<ParserOptions> {
    // Copy the options into a new struct, before we validate the paths
    let mut validated_options = ParserOptions {
        attach_metadata_to_raws: options.attach_metadata_to_raws,
        locations_to_parse: options.locations_to_parse.clone(),
        raws_to_parse: options.raws_to_parse.clone(),
        serialize_result_to_json: options.serialize_result_to_json,
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
                log::error!(
                    "options_validator: Unable to canonicalize Dwarf Fortress path!\n{:?}\n{:?}",
                    target_path,
                    e
                );
                return None;
            }
        };

        if !target_path.exists() {
            log::error!(
                "options_validator: Provided Dwarf Fortress path for doesn't exist!\n{}",
                target_path.display()
            );
            return None;
        }

        if !target_path.is_dir() {
            log::error!(
                "options_validator: Dwarf Fortress path needs to be a directory!\n{}",
                target_path.display()
            );
            return None;
        }

        validated_options.dwarf_fortress_directory = target_path.clone();
    }

    // Validate any raw file paths
    for raw_file_path in &options.raw_files_to_parse {
        if !raw_file_path.exists() {
            log::warn!(
                "options_validator: Provided raw file path doesn't exist!\n{}",
                raw_file_path.display()
            );
        } else if !raw_file_path.is_file() {
            log::warn!(
                "options_validator: Provided raw file path needs to be a file!\n{}",
                raw_file_path.display()
            );
        } else {
            // Add the canonicalized path to the raw file
            let raw_file_path = raw_file_path.canonicalize().unwrap_or_else(|e| {
                log::error!(
                    "options_validator: Unable to canonicalize raw file path!\n{:?}",
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
            log::error!(
                "options_validator: Provided raw module path doesn't exist!\n{}",
                raw_module_path.display()
            );
        } else if !raw_module_path.is_dir() {
            log::error!(
                "options_validator: Provided raw module path needs to be a directory!\n{}",
                raw_module_path.display()
            );
        } else {
            // Add the canonicalized path to the module
            let raw_module_path = raw_module_path.canonicalize().unwrap_or_else(|e| {
                log::error!(
                    "options_validator: Unable to canonicalize raw module path!\n{:?}",
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
            log::error!(
                "options_validator: Provided legends export path doesn't exist!\n{}",
                legends_export_path.display()
            );
        } else if !legends_export_path.is_file() {
            log::error!(
                "options_validator: Provided legends export path needs to be a file!\n{}",
                legends_export_path.display()
            );
        } else {
            // Add the canonicalized path to the legends export
            let legends_export_path = legends_export_path.canonicalize().unwrap_or_else(|e| {
                log::error!(
                    "options_validator: Unable to canonicalize legends export path!\n{:?}",
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
            log::error!(
                "options_validator: Provided module info file path doesn't exist!\n{}",
                module_info_file_path.display()
            );
        } else if !module_info_file_path.is_file() {
            log::error!(
                "options_validator: Provided module info file path needs to be a file!\n{}",
                module_info_file_path.display()
            );
        } else {
            // Add the canonicalized path to the module info file
            let module_info_file_path = module_info_file_path.canonicalize().unwrap_or_else(|e| {
                log::error!(
                    "options_validator: Unable to canonicalize module info file path!\n{:?}",
                    e
                );
                module_info_file_path.clone()
            });
            validated_options
                .module_info_files_to_parse
                .push(module_info_file_path);
        }
    }

    Some(validated_options)
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

pub fn try_get_file<P: AsRef<Path>>(file_path: &P) -> Option<File> {
    let caller = "File Exists Validator";
    // Validate file exists
    if !file_path.as_ref().exists() {
        log::error!(
            "{} - Path doesn't exist {}",
            caller,
            file_path.as_ref().display()
        );
        return None;
    }
    if !file_path.as_ref().is_file() {
        log::error!(
            "{} - Path does not point to a file {}",
            caller,
            file_path.as_ref().display(),
        );
        return None;
    }

    // Open the file
    let Ok(file) = File::open(file_path) else {
        log::error!(
            "{} - Unable to open file {}",
            caller,
            file_path.as_ref().display()
        );
        return None;
    };

    Some(file)
}
