use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::{ModuleInfoFile, ObjectType, RawModuleLocation};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
#[derive(ts_rs::TS)]
#[ts(export, rename = "RawMetadata")]
/// The `RawMetadata` struct represents metadata about a raw module in Rust, including its name,
/// version, file path, identifier, object type, module location, and visibility status.
///
/// Properties:
///
/// * `module_name`: The name of the raw module the raw is from.
/// * `module_version`: The version of the raw module the raw is from.
/// * `raw_file_path`: The `raw_file_path` property is a string that represents the path to the file
/// containing the raw data. It specifies the location of the file on the file system.
/// * `raw_identifier`: The raw identifier is a unique identifier for the raw data. It is typically
/// found at the top of the raw text file and is used to identify and reference the specific raw data.
/// * `object_type`: The `object_type` property represents the type of the raw data. It could be a
/// creature, plant, or any other type specified in the raw text file.
/// * `raw_module_location`: The `raw_module_location` property represents the location of the owning
/// raw module. It can have one of the following values:
///
///     - `RawModuleLocation::InstalledMods`: The raw module is located in the `installed_mods` folder.
///     - `RawModuleLocation::Mods`: The raw module is located in the `mods` folder.
///     - `RawModuleLocation::Vanilla`: The raw module is located in the `vanilla` folder.
///
/// * `hidden`: The `hidden` property is a boolean value that indicates whether the raw metadata should
/// be hidden or not when exporting. By default, it is set to `true`, meaning that the raw metadata will
/// be hidden unless specified in the `ParsingOptions` struct.
pub struct Metadata {
    // The name of the raw module the raw is from.
    module_name: String,
    // The version of the raw module the raw is from.
    module_version: String,
    // The path to the file containing the raw.
    raw_file_path: String,
    // The raw identifier (as described at the top of the raw text file).
    raw_identifier: String,
    // The type of raw (creature, plant, etc).
    // Example: [OBJECT:TYPE]
    object_type: ObjectType,
    // The location of the owning raw module
    // i.e. installed_mods, mods, or vanilla
    raw_module_location: RawModuleLocation,
    // Optionally hide or unhide from exporting
    // By default will be hidden
    #[serde(skip)]
    hidden: bool,
}

impl Metadata {
    pub fn new<P: AsRef<Path>>(
        module_info: &ModuleInfoFile,
        object_type: &ObjectType,
        raw_identifier: &str,
        raw_file_path: &P,
        attach_metadata_to_raws: bool,
    ) -> Self {
        Self {
            module_name: module_info.get_name(),
            module_version: module_info.get_version(),
            raw_file_path: String::from(raw_file_path.as_ref().to_str().unwrap_or_default()),
            raw_identifier: String::from(raw_identifier),
            object_type: object_type.clone(),
            raw_module_location: module_info.get_location(),
            hidden: !attach_metadata_to_raws,
        }
    }
    /// (Hidden from export) Used only for serialization
    pub fn is_hidden(&self) -> bool {
        self.hidden
    }
    /// Get the identifier of the raw file the raw is from.
    pub fn get_raw_identifier(&self) -> &str {
        &self.raw_identifier
    }
    /// Get the name of the module the raw is from.
    pub fn get_module_name(&self) -> &str {
        &self.module_name
    }
    /// Get the (numeric) version of the module the raw is from.
    pub fn get_module_numerical_version(&self) -> &str {
        &self.module_version
    }
    /// Get the (string) version of the module the raw is from.
    pub fn get_module_version(&self) -> &str {
        &self.module_version
    }
    /// Get the full path to the raw file the raw is from.
    pub fn get_raw_file_path(&self) -> &str {
        &self.raw_file_path
    }
    /// Get the location of the owning raw module.
    pub fn get_location(&self) -> &RawModuleLocation {
        &self.raw_module_location
    }
}