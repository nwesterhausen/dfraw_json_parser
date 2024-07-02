use std::path::{Path, PathBuf};

use tracing::{debug, info};
use walkdir::DirEntry;

use crate::{
    metadata::{ParserOptions, RawModuleLocation},
    parser::parse_module,
    reader::{FileParseResult, UnprocessedRaw},
    traits::RawObject,
    utilities::subdirectories,
    ParserError,
};

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
pub fn parse_location<P: AsRef<Path>>(
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
