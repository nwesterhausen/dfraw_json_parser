use serde::{Deserialize, Serialize};
use specta::Type;

use crate::RawModuleLocation;

/// Details about the current file being parsed, specifically where and what it is.
#[derive(Serialize, Deserialize, Debug, Clone, Default,Type)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::module_name_repetitions)]
pub struct ProgressDetails {
    /// The current module location (only for stuff parsed via a specified location, otherwise is `RawModuleLocation::Unknown`)
    pub location: RawModuleLocation,
    /// The module the current file is in (if any)
    pub module: Option<String>,
    /// The parsed name of the file being parsed (if it can be determined)
    pub raw_file: Option<String>,
    /// The location of the file being parsed (if it can be determined)
    pub file_location: Option<String>,
}
