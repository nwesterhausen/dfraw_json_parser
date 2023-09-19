use std::{
    fs::File,
    io::{BufWriter, Write},
    path::{Path, PathBuf},
};

use walkdir::WalkDir;

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

/// It takes a string of json and writes it to a file, wrapping it in square brackets to make it a valid
/// json array
///
/// Arguments:
///
/// * `parsed_json_string`: String
/// * `out_filepath`: Path
pub fn write_json_string_to_file<P: AsRef<Path>>(parsed_json_string: &String, out_filepath: &P) {
    log::info!("Saving json to to {:?}", out_filepath.as_ref().display());

    let out_file = match File::create(out_filepath) {
        Ok(f) => f,
        Err(e) => {
            log::error!(
                "Unable to open {} for writing \n{:?}",
                out_filepath.as_ref().display(),
                e
            );
            return;
        }
    };

    let mut stream = BufWriter::new(out_file);
    let write_error = &format!(
        "Unable to write to {}",
        out_filepath.as_ref().to_string_lossy()
    );
    match write!(stream, "[") {
        Ok(_x) => (),
        Err(e) => {
            log::error!("{}\n{:?}", write_error, e);
            return;
        }
    };

    match write!(stream, "{parsed_json_string}") {
        Ok(_x) => (),
        Err(e) => {
            log::error!("{}\n{:?}", write_error, e);
            return;
        }
    };

    match write!(stream, "]") {
        Ok(_x) => (),
        Err(e) => {
            log::error!("{}\n{:?}", write_error, e);
            return;
        }
    };
    match stream.flush() {
        Ok(_x) => (),
        Err(e) => {
            log::error!("{}\n{:?}", write_error, e);
        }
    };
}

#[allow(dead_code)]
/// It takes a string of json and writes it to a file, wrapping it in square brackets to make it a valid
/// json array
///
/// Arguments:
///
/// * `parsed_json_string_vec`: String
/// * `out_filepath`: Path
pub fn write_json_string_vec_to_file<P: AsRef<Path>>(
    parsed_json_string_vec: &Vec<String>,
    out_filepath: &P,
) {
    log::info!("Saving json to to {:?}", out_filepath.as_ref().display());

    let out_file = match File::create(out_filepath) {
        Ok(f) => f,
        Err(e) => {
            log::error!(
                "Unable to open {} for writing \n{:?}",
                out_filepath.as_ref().display(),
                e
            );
            return;
        }
    };

    let mut stream = BufWriter::new(out_file);
    let write_error = &format!(
        "Unable to write to {}",
        out_filepath.as_ref().to_string_lossy()
    );
    match write!(stream, "[") {
        Ok(_x) => (),
        Err(e) => {
            log::error!("{}\n{:?}", write_error, e);
            return;
        }
    };

    for parsed_json_string in parsed_json_string_vec {
        match write!(stream, "{parsed_json_string}") {
            Ok(_x) => (),
            Err(e) => {
                log::error!("{}\n{:?}", write_error, e);
                return;
            }
        };
    }

    match write!(stream, "]") {
        Ok(_x) => (),
        Err(e) => {
            log::error!("{}\n{:?}", write_error, e);
            return;
        }
    };
    match stream.flush() {
        Ok(_x) => (),
        Err(e) => {
            log::error!("{}\n{:?}", write_error, e);
        }
    };
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

/// It takes a string of raws-style strings (i.e. [content]) and writes it to file, one per line.
///
/// Arguments:
///
/// * `parsed_raws_string_vec`: String
/// * `out_filepath`: Path
pub fn write_raws_string_vec_to_file<P: AsRef<Path>>(
    parsed_raws_string_vec: &Vec<String>,
    out_filepath: &P,
) {
    log::info!("Saving raws to to {:?}", out_filepath.as_ref().display());

    let out_file = match File::create(out_filepath) {
        Ok(f) => f,
        Err(e) => {
            log::error!(
                "Unable to open {} for writing \n{:?}",
                out_filepath.as_ref().display(),
                e
            );
            return;
        }
    };

    let mut stream = BufWriter::new(out_file);
    let write_error = &format!(
        "Unable to write to {}",
        out_filepath.as_ref().to_string_lossy()
    );

    for parsed_json_string in parsed_raws_string_vec {
        match writeln!(stream, "{parsed_json_string}") {
            Ok(_x) => (),
            Err(e) => {
                log::error!("{}\n{:?}", write_error, e);
                return;
            }
        };
    }

    match stream.flush() {
        Ok(_x) => (),
        Err(e) => {
            log::error!("{}\n{:?}", write_error, e);
        }
    };
}
