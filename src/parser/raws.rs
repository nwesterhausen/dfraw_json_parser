use std::{any::Any, path::Path};

use serde::{Deserialize, Serialize};

use super::{
    mod_info_file::ModuleInfoFile, object_types::ObjectType, raw_locations::RawModuleLocation,
};

#[typetag::serde]
pub trait RawObject: RawObjectToAny {
    fn get_metadata(&self) -> &RawMetadata;
    fn get_identifier(&self) -> &str;
    fn is_empty(&self) -> bool;
    fn get_type(&self) -> &ObjectType;
    fn parse_tag(&mut self, key: &str, value: &str);
}

pub trait RawObjectToAny: 'static {
    fn as_any(&self) -> &dyn Any;
}

impl<T: 'static> RawObjectToAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

// The metadata for a DF Raw. This includes information about the raw module
// the raw is from, and which file contains the raw.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct RawMetadata {
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
    hidden: bool,
}

impl RawMetadata {
    pub fn new<P: AsRef<Path>>(
        module_info: &ModuleInfoFile,
        object_type: &ObjectType,
        raw_identifier: &str,
        raw_file_path: &P,
    ) -> Self {
        Self {
            module_name: module_info.get_name(),
            module_version: module_info.get_version(),
            raw_file_path: String::from(raw_file_path.as_ref().to_str().unwrap_or_default()),
            raw_identifier: String::from(raw_identifier),
            object_type: object_type.clone(),
            raw_module_location: module_info.get_location(),
            hidden: true,
        }
    }
    pub(crate) fn is_hidden(&self) -> bool {
        self.hidden
    }
}
